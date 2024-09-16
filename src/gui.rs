use std::path::PathBuf;

use crate::{cli::Cli, Opts};

use anyhow::anyhow;
use eframe::{egui, App};
use egui::CentralPanel;
use rfd::FileDialog;

#[derive(Default, Debug)]
pub struct GUI{
    lua_path: PathBuf,
    error : String,

    #[allow(dead_code)]
    opts: Opts,
}

impl App for GUI{
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx,|ui| {
            let grid = egui::Grid::new("id");
            grid.show(ui, |ui|{
                ui.label("codigo:");
                ui.label( self.lua_path.to_str().unwrap() );
                if ui.button("...").clicked(){
                    match FileDialog::new().pick_file(){
                        Some(v) => self.lua_path = v,
                        _ => {},
                    }
                };
                ui.end_row();

                ui.label("baraja:");
                ui.label( self.opts.deck.to_str().unwrap() );
                if ui.button("...").clicked(){
                    match FileDialog::new().pick_folder(){
                        Some(v) => self.opts.deck = v,
                        _ => {},
                    }
                };
                ui.end_row();

                ui.label("cartas:");
                ui.label( self.opts.output.to_str().unwrap() );
                if ui.button("...").clicked(){
                    match FileDialog::new().pick_folder(){
                        Some(v) => self.opts.output= v,
                        _ => {},
                    }
                };

                ui.end_row();

                ui.label("nombre de baraja:");
                ui.text_edit_singleline( &mut self.opts.in_format );
                ui.end_row();

                ui.label("nombre de carta:");
                ui.text_edit_singleline( &mut self.opts.out_format );
                ui.end_row();

                if ui.button("run").clicked(){
                    let opts = self.opts.clone();
                    match Cli::config(opts, self.lua_path.clone()).and_then(Cli::run){
                        Err(e) => {
                            println!("{e}");
                            self.error = format!("error: {e}");
                        },
                        _ => self.error = String::from("no errors!"),
                    }
                }
            });


            ui.label( &self.error );
        });
    }
}

impl GUI {
    pub fn new(opts: Opts) -> Self{
        let new = GUI{opts, ..Default::default()};

        return new;
    }

    pub fn run(self) -> anyhow::Result<()>{
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
