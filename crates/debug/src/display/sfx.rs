use game::{
    cs::CSSfxImp,
    gxffx::{FxrListNode, FxrWrapper, GXFfxGraphicsResourceManager, GXFfxSceneCtrl},
};
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for CSSfxImp {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("CSSfxImp", TreeNodeFlags::empty()) {
            ui.text(format!("CSSfxImp: {:#01x}", self as *const _ as usize));
            ui.indent();
            self.scene_ctrl.render_debug(ui);
            ui.unindent();
        }
    }
}

impl DebugDisplay for GXFfxSceneCtrl {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Scene Ctrl", TreeNodeFlags::empty()) {
            ui.text(format!(
                "graphics_resource_manager: {:#01x}",
                self.graphics_resource_manager as *const _ as usize
            ));
            ui.indent();
            self.graphics_resource_manager.render_debug(ui);
            ui.unindent();
        }
    }
}

impl DebugDisplay for GXFfxGraphicsResourceManager {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Graphics Resource Manager", TreeNodeFlags::empty()) {
            ui.indent();
            render_graphics_resource_manager(
                self.resource_container.fxr_definitions.iter().map(|f| f),
                ui,
            );
            ui.unindent();
        }
    }
}

// TODO: Address crashing
fn render_graphics_resource_manager<'a>(
    fxr_nodes: impl Iterator<Item = &'a FxrListNode>,
    ui: &&mut Ui,
) {
    if let Some(_t) = ui.begin_table_header(
        "gx-ffx-graphics-resource-manager",
        [
            TableColumnSetup::new("ID"),
            TableColumnSetup::new("FXR Ptr"),
        ],
    ) {
        fxr_nodes.for_each(|fxr_node| {
            fxr_node.render_debug(ui);
        });
    }
}

impl DebugDisplay for FxrWrapper {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("{:#01x}", self.fxr));
    }
}

impl DebugDisplay for FxrListNode {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.table_next_column();
        ui.text(format!("{}", self.id));
        ui.table_next_column();
        self.fxr_wrapper.render_debug(ui);
    }
}
