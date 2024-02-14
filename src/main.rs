use eframe::{egui, Frame};
use egui::Context;
use std::default::Default;
use eframe::egui::{Ui, Vec2};

pub struct AppSettings
{
    window_dim: [f32; 2],
    window_min_dim: [f32; 2],
    window_max_dim: [f32; 2],
    title:  String
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            window_dim: [800.0,800.0],
            window_min_dim: [400.0,400.0],
            window_max_dim: [1600.0,1600.0],
            title: "RF Control".to_owned(),
        }
    }
}
impl AppSettings { pub fn get() -> Self { Default::default()} }
pub struct MainGUI
{
    //add settings
}

impl Default for MainGUI
{
    fn default() -> Self{
        Self{
            //initiate settings
        }
    }
}

impl MainGUI {
    pub fn new (cc: &eframe::CreationContext<'_>) -> Self {Default::default()}// init constructor
    //other functions
    fn menu_bar(ui: &mut Ui)
    {
        //menu bar
    }

    fn main_window(ui: &mut Ui)
    {
        //main content
       Self::footer(ui);

    }

    fn footer(ui: &mut Ui)
    {
        //footer
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            egui::warn_if_debug_build(ui);
        });
    }

}
impl eframe::App for MainGUI
{
    fn update(&mut self, ctx: &Context, frame: &mut Frame)
    {
        //do gui

        //Topbar
        egui::TopBottomPanel::top("top_panel").show(ctx, | ui | Self::menu_bar);

        //main content
        egui::CentralPanel::default().show(ctx, | ui | Self::main_window);

    }

}
fn main() -> eframe::Result<()>
{
    let app_config = AppSettings::get();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(app_config.window_dim)
            .with_min_inner_size(app_config.window_min_dim)
            .with_max_inner_size(app_config.window_max_dim),
        ..Default::default()
    };

    //run gui
    eframe::run_native(app_config.title.as_str(),
                       native_options,
                       Box::new(| cc| Box::new(MainGUI::new(cc))))
}
