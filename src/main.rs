#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod geodata;

use eframe::egui;

const GEOSITE_URL: &str =
    "https://github.com/Loyalsoldier/v2ray-rules-dat/releases/latest/download/geosite.dat";

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "GeoSite Reader",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    geosite: geodata::GeoSiteList,
}

impl Default for MyApp {
    fn default() -> Self {
        let request = ehttp::Request::get(GEOSITE_URL);
        let result = ehttp::fetch_blocking(&request);
        let geosite_data = geodata::deserialize_geosite(&result.unwrap().bytes).unwrap();
        Self {
            geosite: geosite_data,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let geosite = &self.geosite;
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::CollapsingHeader::new(GEOSITE_URL)
                    .default_open(false)
                    .show(ui, |ui| {
                        egui::Grid::new(GEOSITE_URL)
                            .spacing(egui::vec2(ui.spacing().item_spacing.x * 2.0, 0.0))
                            .show(ui, |ui| {
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
                                                        ui.label(&ele2.value);
                                                        ui.end_row()
                                                    }
                                                })
                                        });
                                    ui.end_row()
                                }
                            })
                    });
            });
        });
    }
}
