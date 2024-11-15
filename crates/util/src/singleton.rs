use dlrf::DLRFSingleton;
use pelite::pattern;
use pelite::pe::Pe;
use pelite::pe::Rva;
use std::collections;
use std::io::Write;
use std::sync;
use std::sync::LazyLock;
use thiserror::Error;

use crate::program::Program;

pub type SingletonMap = collections::HashMap<String, usize>;
static SINGLETON_MAP: sync::OnceLock<SingletonMap> = sync::OnceLock::new();

#[derive(Error, Debug)]
pub enum SingletonMapError {
    #[error("Could not find section {0}.")]
    Section(&'static str),
    #[error("Could not parse the discovered singleton's name.")]
    MalformedName,
}

#[derive(Error, Debug)]
pub enum LookupError {
    #[error("Singleton was not found.")]
    NotFound,
    #[error("Could not create the singleton map. {0}")]
    SingletonMapCreation(SingletonMapError),
}

/// Looks up instances of singleton instances by their name.
/// Some singletons aren't necessarily always instanciated and available.
/// Discovered singletons are cached so invokes after the first will be much faster.
///
/// # SAFETY
/// All bets are off if you grab a ref to a singleton without understanding the conditions around
/// shared access like multithreaded access and locking. The best place to use this is from the
/// task runtime after extensively considering what phases of the game loop update or read a
/// singleton.
pub unsafe fn get_instance<T: DLRFSingleton>() -> Result<Option<&'static mut T>, LookupError> {
    let table = SINGLETON_MAP.get_or_init(move || {
        let program = unsafe { Program::current() };

        build_singleton_table(&program)
            .map_err(LookupError::SingletonMapCreation)
            .expect("Could not create singleton map")
    });

    let ptr = table
        .get(T::DLRF_NAME)
        .map(usize::to_owned)
        .ok_or(LookupError::NotFound)?;

    unsafe { Ok((*(ptr as *const *mut T)).as_mut()) }
}

const NULL_CHECK_PATTERN: &str = concat!(
    //  MOV REG, [MEM]
    "48 8b 0d $ { ' }",
    //  TEST REG, REG
    "48 85 ? ",
    // JNZ +2e
    "75 ? ",
    // LEA RCX, [runtime_class_metadata]
    "48 8d 0d $ { ' }",
    // 19 CALL get_singleton_name
    "e8 $ { ' }"
);

/// Builds a table of all the singletons. It does so by looking for null checks
/// in the game by using an instance pattern. It then cycles over all
/// candidates and vets the involved pointers. We expect a pointer to the
/// instance's static, a pointer to the reflection metadata and a pointer to
/// the get_singleton_name fn. Once all checks out we call get_singleton_name
/// with the metadata to obtain the instance's type name.
pub fn build_singleton_table(program: &Program) -> Result<SingletonMap, SingletonMapError> {
    let text_range = program
        .section_headers()
        .by_name(".text")
        .ok_or(SingletonMapError::Section(".text"))?
        .virtual_range();

    let data_range = program
        .section_headers()
        .by_name(".data")
        .ok_or(SingletonMapError::Section(".data"))?
        .virtual_range();

    let pattern = pattern::parse(NULL_CHECK_PATTERN).unwrap();
    let mut matches = program.scanner().matches_code(&pattern);
    let mut captures: [Rva; 4] = [Rva::default(); 4];
    let mut results: SingletonMap = Default::default();

    while matches.next(&mut captures) {
        let static_rva = captures[1];
        let metadata_rva = captures[2];
        let get_reflection_name_rva = captures[3];

        // Check if all RVAs are plausible.
        if !data_range.contains(&static_rva)
            || !data_range.contains(&metadata_rva)
            || !text_range.contains(&get_reflection_name_rva)
        {
            continue;
        }

        let metadata = program.rva_to_va(metadata_rva).unwrap();
        let get_singleton_name: extern "C" fn(u64) -> *const i8 =
            unsafe { std::mem::transmute(program.rva_to_va(get_reflection_name_rva).unwrap()) };

        let cstr = unsafe { std::ffi::CStr::from_ptr(get_singleton_name(metadata)) };
        let singleton_name = cstr
            .to_str()
            .map_err(|_| SingletonMapError::MalformedName)?
            .to_string();

        let singleton_va = program.rva_to_va(static_rva).unwrap();
        results.insert(singleton_name, singleton_va as usize);
    }

    Ok(results)
}
