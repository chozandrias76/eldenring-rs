use std::{mem::transmute, ptr::NonNull};

use game::{
    cs::{CSFileImp, MsbFileCap, MsbRepository},
    dlio::{
        DLFileDeviceBase, DLFileDeviceManager, OverrideProxyFileDevice,
    },
    dlkr::DLAllocatorVmt,
};
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags};

use super::DebugDisplay;

impl DebugDisplay for MsbRepository {
    fn render_debug(&self, ui: &&mut hudhook::imgui::Ui) {
        if ui.collapsing_header("Resources", TreeNodeFlags::empty()) {
            ui.indent();
            if let Some(_t) =
                ui.begin_table_header("msb-repository-rescaps", [TableColumnSetup::new("Name")])
            {
                for msb in self.res_rep.res_cap_holder.entries() {
                    ui.table_next_column();
                    ui.text(msb.file_cap.res_cap.name.to_string());
                }
            }
        }

        if ui.button("Test MSB Load") {
            let file_device_manager =
                unsafe { &mut *(0x1448464c0usize as *mut DLFileDeviceManager) };

            file_device_manager.mutex.lock();
            file_device_manager
                .devices
                .items()
                .iter_mut()
                .for_each(|d| {
                    *d = NonNull::new(Box::leak(Box::new(OverrideProxyFileDevice::new(
                        d.to_owned(),
                    ))) as *mut OverrideProxyFileDevice
                        as *mut DLFileDeviceBase)
                    .unwrap();
                });
            file_device_manager.mutex.unlock();
        }
        ui.unindent();
    }
}

fn load_msb(path: &str) {
    // Get some memory
    // let allocator: &mut DLAllocatorBase = unsafe { transmute(0x143b405b8usize) };
    // let allocation = allocator.allocate_aligned(0xC8, 8);
    // tracing::info!("Allocation: {allocation:x?}");

    // Construct MsbFileCap
    //
    // FUN_1401f35e0(GLOBAL_CSFile,virtualPath,param_1->field1_0xb0);
    let cs_file_imp: &mut &mut CSFileImp = unsafe { transmute(0x143d5b0f8usize) };
    let create_msb_filecap: extern "C" fn(
        &mut CSFileImp,
        *const u16,
        usize,
    ) -> NonNull<MsbFileCap> = unsafe { transmute(0x1401f35e0usize) };

    let mut string_bytes = path.encode_utf16().collect::<Vec<_>>();
    string_bytes.push(0x0);

    let msb = unsafe { create_msb_filecap(cs_file_imp, string_bytes.as_ptr(), 0).as_mut() };
    tracing::info!(
        "File cap {} - {:x?}",
        msb.file_cap.res_cap.name.to_string(),
        msb as *mut _ as usize
    );
}
