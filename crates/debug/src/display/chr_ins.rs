use game::cs::{ChrAsm, ChrIns, ChrInsModuleContainer, ChrPhysicsModule, FieldInsHandle, PlayerGameData, PlayerIns};
use hudhook::imgui::{TreeNodeFlags, Ui};

use super::DebugDisplay;


impl DebugDisplay for PlayerIns<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        self.chr_ins.render_debug(ui);

        if ui.collapsing_header("ChrAsm", TreeNodeFlags::empty()) {
            self.chr_asm.render_debug(ui);
        }

        if ui.collapsing_header("PlayerGameData", TreeNodeFlags::empty()) {
            self.player_game_data.render_debug(ui);
        }

        if ui.collapsing_header("Map relative position", TreeNodeFlags::empty()) {
            self.map_relative_position.render_debug(ui);
        }
    }
}

impl DebugDisplay for ChrAsm {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Arm style: {:?}", self.arm_style));
        ui.text(format!("Left-hand weapon slot: {:?}", self.left_weapon_slot));
        ui.text(format!("Right-hand weapon slot: {:?}", self.right_weapon_slot));
        ui.text(format!("Left-hand arrow slot: {:?}", self.left_arrow_slot));
        ui.text(format!("Right-hand arrow slot: {:?}", self.right_weapon_slot));
        ui.text(format!("Left-hand bolt slot: {:?}", self.left_bolt_slot));
        ui.text(format!("Right-hand bolt slot: {:?}", self.right_bolt_slot));

        for (i, e) in self.gaitem_handles.iter().enumerate() {
            ui.text(format!("Gaitem {}: {:x?}", i, e));
        }

        for (i, e) in self.equipment_param_ids.iter().enumerate() {
            ui.text(format!("Equipment param ID {}: {:?}", i, e));
        }
    }
}

impl DebugDisplay for PlayerGameData {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Furlcalling Finger Active: {:?}", self.furlcalling_finger_remedy_active));
        ui.text(format!("Rune Arc Active: {:?}", self.rune_arc_active));
        ui.text(format!("White Ring Active: {:?}", self.white_ring_active));
        ui.text(format!("Blue Ring Active: {:?}", self.blue_ring_active));
    }
}

impl DebugDisplay for ChrIns<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        self.field_ins_handle.render_debug(ui);

        ui.text(format!("Map ID 1: {:?}", self.map_id_1));
        ui.text(format!("Map ID origin 1: {}", self.map_id_origin_1));
        ui.text(format!("Map ID 2: {:?}", self.map_id_2));
        ui.text(format!("Map ID origin 2: {}", self.map_id_origin_2));
        ui.text(format!("Last used item?: {}", self.last_used_item));
        ui.text(format!("Character ID: {}", self.character_id));

        if ui.collapsing_header("Modules", TreeNodeFlags::empty()) {
            self.module_container.render_debug(ui);
        }
    }
}

impl DebugDisplay for FieldInsHandle {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Field Ins ID: {}", self.instance_id));
        ui.text(format!("Field Ins map ID: {:?}", self.map_id));
    }
}

impl DebugDisplay for ChrInsModuleContainer<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Physics", TreeNodeFlags::empty()) {
            self.physics.render_debug(ui);
        }
    }
}

impl DebugDisplay for ChrPhysicsModule<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Unk70 position", TreeNodeFlags::empty()) {
            self.unk70_position.render_debug(ui);
        }

        if ui.collapsing_header("Unk80 position", TreeNodeFlags::empty()) {
            self.unk80_position.render_debug(ui);
        }
    }
}
