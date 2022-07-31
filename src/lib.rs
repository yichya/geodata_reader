mod geodata;

use eframe::{egui, epi};
use std::sync::{Arc, Mutex};

#[cfg(not(target_arch = "wasm32"))]
const GEOSITE_URL: &str =
    "https://github.com/Loyalsoldier/v2ray-rules-dat/releases/latest/download/geosite.dat";

enum Download {
    None,
    InProgress,
    Done(ehttp::Result<geodata::GeoSiteList>),
}

pub struct MyApp {
    geosite: Arc<Mutex<Download>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            geosite: Arc::new(Mutex::new(Download::None)),
        }
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let mut trigger_download = false;
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let gm: &Download = &self.geosite.lock().unwrap();
                match gm {
                    Download::Done(r) => match r {
                        Ok(geosite) => {
                            for (index, element) in geosite.entry.iter().enumerate() {
                                egui::CollapsingHeader::new(&element.country_code)
                                    .default_open(false)
                                    .show(ui, |ui| {
                                        egui::Grid::new(index)
                                            .spacing(egui::vec2(
                                                ui.spacing().item_spacing.x * 2.0,
                                                0.0,
                                            ))
                                            .show(ui, |ui| {
                                                for ele2 in &geosite.entry[index].domain {
                                                    ui.monospace(&ele2.value);
                                                    ui.end_row()
                                                }
                                            })
                                    });
                                ui.end_row()
                            }
                        }
                        Err(_error) => {
                            egui::CollapsingHeader::new("Download Failed")
                                .default_open(false)
                                .show(ui, |ui| {
                                    ui.end_row();
                                });
                        }
                    },
                    Download::InProgress => {
                        egui::CollapsingHeader::new("Downloading...")
                            .default_open(false)
                            .show(ui, |ui| {
                                ui.end_row();
                            });
                    }
                    Download::None => {
                        trigger_download = true;
                    }
                }
            });
        });

        if trigger_download {
            let request = ehttp::Request::get(GEOSITE_URL);
            let download_store = self.geosite.clone();
            *download_store.lock().unwrap() = Download::InProgress;
            let frame = frame.clone();
            ehttp::fetch(request, move |response| {
                *download_store.lock().unwrap() = Download::Done(Ok(geodata::deserialize_geosite(
                    &response.unwrap().bytes,
                )
                .unwrap()));
                frame.request_repaint();
            });
        }
    }

    fn name(&self) -> &str {
        "geodata_reader"
    }

    #[cfg(target_arch = "wasm32")]
    fn max_size_points(&self) -> Vec2 {
        Vec2::new(940.0, 750.0)
    }
}

// ----------------------------------------------------------------------------
#[cfg(target_arch = "wasm32")]
const GEOSITE_URL: &str = "/geosite.dat";

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
use egui::Vec2;

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
    let app = MyApp::default();
    eframe::start_web(canvas_id, Box::new(app))
}
