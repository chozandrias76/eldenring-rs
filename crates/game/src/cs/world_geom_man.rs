use std::fmt::Formatter;

use crate::{DLRFLocatable, Tree, Vector};

use super::{FieldInsHandle, MapId};

#[repr(C)]
/// Source of name: RTTI
pub struct CSWorldGeomMan<'a> {
    pub vftable: usize,
    pub unk8: usize,
    pub world_info_owner: usize,
    /// A tree of geometry containers per map.  
    pub map_geometry: Tree<CSWorldGeomManMapData<'a>>,
    /// Seemingly points to the current overlay world tile's map data
    pub curent_99_map_data: &'a CSWorldGeomManMapData<'a>,
}

impl DLRFLocatable for CSWorldGeomMan<'_> {
    const DLRF_NAME: &'static str = "CSWorldGeomMan";
}

#[repr(C)]
/// Seems to host any spawned geometry for a given map.
pub struct CSWorldGeomManMapData<'a> {
    /// The map ID this container hosts the assets for.
    pub map_id: MapId,
    /// Might be padding?
    unk4: u32,
    pub world_block_info: usize,
    unk10: [u8; 0x278],
    /// Holds refs to all geometry instances fopr this map (both dynamic and static). 
    pub geom_ins_vector: Vector<&'a CSWorldGeomIns<'a>>,
    unk2a8: [u8; 0x88],
    /// Seems to be the next field ins index that will be assiged.
    pub next_geom_ins_field_ins_index: u32,
    /// Seems to indicate if the geometry_ins vector has reached some hardcoded capacity?
    pub reached_geom_ins_vector_capacity: bool,
    _pad335: [u8; 3],
    unk338: [u8; 0x50],
    pub sos_sign_geometry: Vector<usize>,
    unk3a8: [u8; 0x300],
    // TODO: need to figure out how big this thing actually is
}

#[repr(C)]
/// Abstract base class for geometry instances.
///
/// Source of name: RTTI
pub struct CSWorldGeomIns<'a> {
    pub vfptr: usize,
    pub field_ins_handle: FieldInsHandle,
    /// Points to the map data hosting this GeomIns.
    pub map_data: &'a CSWorldGeomManMapData<'a>,
    /// Points to the world placement data for this geometry instance.
    pub info: CSWorldGeomInfo<'a>,
    // TODO: fill me
    pub unk1a8: [u8; 0x288],
}

#[repr(C)]
/// Holds the asset details in regard to placement in the world, drawing, etc.
///
/// Source of name: "..\\..\\Source\\Game\\Geometry\\CSWorldGeomInfo.cpp" in exception.
pub struct CSWorldGeomInfo<'a> {
    /// Points to the map data hosting the GeomIns for this info struct.
    pub map_data: &'a CSWorldGeomManMapData<'a>,
    /// Points to the param row this geometry instance uses. 
    pub asset_geometry_param: usize,
    unk10: u32,
    // Might be padding
    unk14: u32,
    pub msb_parts_geom: CSMsbPartsGeom,
    unk68: u32,
    unk6c: u32,
    unk70: u32,
    unk74: u32,
    unk78: CSWorldGeomInfoUnk,
    unke0: CSWorldGeomInfoUnk,
    unk148: u16,
    unk14a: u8,
    unk14b: u8,
    /// Source of name: Params being copied over
    pub far_clip_distance: f32,
    /// Source of name: Params being copied over
    pub distant_view_model_border_dist: f32,
    /// Source of name: Params being copied over
    pub distant_view_model_play_dist: f32,
    /// Source of name: Params being copied over
    pub limted_activate_border_dist_for_grid: f32,
    /// Source of name: Params being copied over
    pub limted_activate_play_dist_for_grid: f32,
    /// Source of name: Params being copied over
    pub z_sort_offset_for_no_far_clip_draw: u32,
    unk164: u32,
    unk168: f32,
    unk16c: f32,
    unk170: f32,
    sound_obj_enable_dist: f32,
    unk178: u8,
    unk179: u8,
    unk17a: u8,
    unk17c: u8,
    /// Source of name: Params being copied over
    pub has_tex_lv01_border_dist: bool,
    /// Source of name: Params being copied over
    pub is_no_far_clip_draw: bool,
    /// Source of name: Params being copied over
    pub is_trace_camera_xz: bool,
    /// Source of name: Params being copied over
    pub forward_draw_envmap_blend_type: bool,
    unk180: u16,
    unk182: u16,
    unk184: u8,
    unk185: u8,
    unk186: u16,
    unk188: usize,
}

#[repr(C)]
pub struct CSWorldGeomInfoUnk {
    unk0: u32,
    unk4: u32,
    unk8: u32,
    unkc: u32,
    unk10: u32,
    unk14: u32,
    unk18: u32,
    unk1c: u32,
    unk20: usize,
    unk28: [u8; 0x38],
    unk60: usize,
}

#[repr(C)]
/// Seems to describe how to draw the MSB part.
pub struct CSMsbPartsGeom {
    pub msb_parts: CSMsbParts,
}

#[repr(C)]
/// Seems to describe how to draw the MSB part.
pub struct CSMsbParts {
    pub vfptr: usize,
    pub draw_flags: u32,
    unkc: u32,
    unk10: usize,
    pub msb_part: usize,
    unk20: [u8; 0x30],
}

#[repr(C)]
/// Used by the game to seperate geometry spawning code (like MSB parser) from the actual GeomIns
/// construction details. 
pub struct GeometrySpawnRequest {
    /// Contains the asset string, ex. "AEG020_370"
    pub asset_string: [u16; 0x20],
    pub unk0x40: u32,
    pub unk0x44: u32,
    pub asset_string_ptr: u64,
    pub unk0x50: u32,
    pub unk0x54: u32,
    pub unk0x58: u32,
    pub unk0x5c: u32,
    pub unk0x60: u32,
    pub unk0x64: u32,
    pub unk0x68: u32,
    pub unk0x6c: u32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
    pub unk0x94: [u8; 0x6C],
}

impl GeometrySpawnRequest {
    pub fn asset(&self) -> String {
        let mut result = String::new();
        for val in self.asset_string.iter() {
            let c: u8 = (*val & 0xFF) as u8;
            if c == 0 {
                break;
            } else {
                result.push(c as char);
            }
        }
        result
    }

    // TODO: guard against strings that are too long
    pub fn set_asset(&mut self, asset: &str) {
        for (i, char) in asset.as_bytes().iter().enumerate() {
            self.asset_string[i] = *char as u16;
        }
    }
}

impl std::fmt::Debug for GeometrySpawnRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GeometrySpawnRequest")
            .field("asset", &self.asset())
            .field("positionX", &self.pos_x)
            .field("positionY", &self.pos_y)
            .field("positionZ", &self.pos_z)
            .field("rotationX", &self.rot_x)
            .field("rotationY", &self.rot_y)
            .field("rotationZ", &self.rot_z)
            .field("scaleX", &self.scale_x)
            .field("scaleY", &self.scale_y)
            .field("scaleZ", &self.scale_z)
            .finish()
    }
}
