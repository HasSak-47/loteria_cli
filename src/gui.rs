use std::path::PathBuf;

use crate::DataOpts;

use anyhow::anyhow;
use eframe::{egui, App};
use egui::CentralPanel;
use rfd::FileDialog;

#[derive(Default, Debug)]
pub struct GUI{
    lua_path: PathBuf,
    error : String,

    #[allow(dead_code)]
    opts: DataOpts,
}

impl App for GUI{
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx,|ui| {
            ui.label("Test");
            ui.label( self.lua_path.to_str().unwrap() );
            if ui.button("...").clicked(){
                self.lua_path = FileDialog::new().pick_file().expect("");
            };
            ui.label( &self.error );
        });
    }
}

impl GUI {
    pub fn new(opts: DataOpts) -> Self{
        GUI{opts, ..Default::default()}
    }

    pub fn run(mut self) -> anyhow::Result<()>{
        let options = eframe::NativeOptions{
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([640.0, 240.0]),
            ..Default::default()
        };

        eframe::run_native(
            "test stuff",
            options,
            Box::new( |_cc| Ok( Box::<GUI>::from(self) ))
        ).map_err(|e| anyhow!("eframe error {e}!"))?;
        Ok(())
    }
}
