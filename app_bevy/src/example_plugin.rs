use bevy::prelude::*;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy_egui::EguiContexts;
use bevy_egui::egui::plot::Plot;

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, show_legacy_pattern);
    }
}

pub (crate) fn show_legacy_pattern(
    mut egui_ctx: EguiContexts,
){
    
    bevy_egui::egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.label("stuff to show zoom");
        ui.button("some button");
        let plot = Plot::new("pattern maker plot drawing")
            //.legend(Legend::default().position(Corner::RightBottom))
            .show_x(true) //shows the marker lines from the cursor
            .show_y(true) //shows the marker lines from the cursor
            .data_aspect(1.0)
            .allow_zoom(true) //(self.drawing_tool_toolbar.drawing_tool == pm_models::DrawingToolKind::Select)
            .allow_drag(true)
            .allow_scroll(true)
            .show_axes([true, true]);
            //.x_axis_formatter(my_x_axis_formatter)
            //.y_axis_formatter(my_y_axis_formatter);
            //.clamp_grid(clamp_grid)
            //.include_x(0.0);
            
            //let response : Response;
            let res = plot.show(ui, |plot_ui| {
                //build out the instructions from the pattern, adding the points and lines as required. 
                //adjust ponit poisitions based on current drag details. 
            });
            //response
    });
}

pub (crate) fn cleanup_legacy_state() {
    
}

