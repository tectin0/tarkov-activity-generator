// #![windows_subsystem = "windows"]

mod maps;
mod weapons;

use std::fmt::Display;

use eframe::{
    egui::{self, RichText},
    NativeOptions,
};
use maps::MAPS;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use weapons::{get_weapons, RandomizeList};

use anyhow::{Context, Result};
use winapi::um::winuser::{self, MessageBoxW};

#[derive(Debug, Default)]
pub(crate) struct Content {
    pub(crate) maps: RandomizeList,
    pub(crate) weapons: RandomizeList,
}

#[derive(Default)]
struct App {
    content: Content,
    map: String,
    weapon: String,
    error: Option<String>,
}

impl App {
    fn new() -> Result<Self> {
        let content = Content {
            maps: RandomizeList(MAPS.iter().map(|x| x.to_string()).collect()),
            weapons: get_weapons()?,
        };

        Ok(App {
            content,
            map: String::new(),
            weapon: String::new(),
            error: None,
        })
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.error.is_some() {
                let popup = egui::Window::new("Error").collapsible(false);

                popup.show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new(self.error.clone().unwrap()).size(20.0));
                        ui.button("Quit").clicked().then(|| {
                            std::process::exit(0);
                        });
                    });
                });
            }

            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.button(RichText::new("Map?").size(40.0))
                        .clicked()
                        .then(|| {
                            self.map = match self.content.maps.random() {
                                Ok(x) => x,
                                Err(e) => {
                                    self.error = Some(e.to_string());
                                    return;
                                }
                            }
                        });

                    ui.label(RichText::new(self.map.clone()).size(40.0));
                });

                ui.horizontal(|ui| {
                    ui.button(RichText::new("Weapon?").size(40.0))
                        .clicked()
                        .then(|| {
                            self.weapon = match self.content.weapons.random() {
                                Ok(x) => x,
                                Err(e) => {
                                    self.error = Some(e.to_string());
                                    return;
                                }
                            }
                        });

                    ui.label(RichText::new(self.weapon.clone()).size(40.0));
                });
            });
        });
    }
}

fn main() -> Result<()> {
    let mut options = NativeOptions::default();
    options.initial_window_size = Some(egui::Vec2::new(500.0, 120.0));
    options.icon_data = Some(load_icon());

    eframe::run_native(
        "Tarkov Activity Generator",
        options,
        Box::new(|_cc| {
            Box::new({
                match App::new() {
                    Ok(app) => app,
                    Err(e) => App {
                        error: Some(e.to_string()),
                        ..Default::default()
                    },
                }
            })
        }),
    )
    .unwrap();

    Ok(())
}

pub(crate) fn load_icon() -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../assets/eft.png"); // provide own icon here
        let image = match image::load_from_memory(icon) {
            Ok(x) => x,
            Err(e) => {
                let l_msg: Vec<u16> =
                    format!("Could not load Icon from memory\n\ncaused by:  {}\0", e)
                        .to_string()
                        .encode_utf16()
                        .collect();
                let l_title: Vec<u16> = "Error\0".encode_utf16().collect();

                unsafe {
                    MessageBoxW(
                        std::ptr::null_mut(),
                        l_msg.as_ptr(),
                        l_title.as_ptr(),
                        winuser::MB_OK | winuser::MB_ICONINFORMATION,
                    );
                }

                image::DynamicImage::new_rgba8(1, 1)
            }
        }
        .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
