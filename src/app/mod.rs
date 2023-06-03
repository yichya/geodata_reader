mod geodata;

use std::sync::{Arc, Mutex};

enum Download {
    None,
    InProgress,
    Done(ehttp::Result<Vec<geodata::GeoSite>>),
}

#[cfg(not(target_arch = "wasm32"))]
const GEOSITE_URL: &str =
    "https://github.com/Loyalsoldier/v2ray-rules-dat/releases/latest/download/geosite.dat";

#[cfg(target_arch = "wasm32")]
const GEOSITE_URL: &str = "geosite.dat";

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GeoDataReader {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    category_search: String,

    #[serde(skip)]
    category_selected: usize,

    #[serde(skip)]
    geosite: Arc<Mutex<Download>>,
}

impl Default for GeoDataReader {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            category_search: "".to_owned(),
            category_selected: 0,
            geosite: Arc::new(Mutex::new(Download::None)),
        }
    }
}

impl GeoDataReader {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for GeoDataReader {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label: _,
            value: _,
            category_search: _,
            category_selected: _,
            geosite: _,
        } = self;

        let mut trigger_download = false;
        egui::Window::new("GeoSite").show(ctx, |ui| {
            let gm: &Download = &self.geosite.lock().unwrap();
            match gm {
                Download::Done(r) => match r {
                    Ok(geosite) => {
                        egui::SidePanel::right("my_right_panel").show_inside(ui, |ui| {
                            ui.label(format!(
                                "{} Total: {}",
                                &geosite[self.category_selected].country_code,
                                &geosite[self.category_selected].domain.len()
                            ));
                            ui.separator();
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                for ele2 in &geosite[self.category_selected].domain {
                                    ui.label(&ele2.value);
                                }
                            });
                        });
                        egui::CentralPanel::default().show_inside(ui, |ui| {
                            egui::TextEdit::singleline(&mut self.category_search)
                                .hint_text("Search Category")
                                .show(ui);
                            ui.separator();

                            egui::ScrollArea::vertical().show(ui, |ui| {
                                for (index, element) in geosite.iter().enumerate() {
                                    if element.country_code.contains(
                                        self.category_search.to_ascii_uppercase().as_str(),
                                    ) {
                                        if ui.link(element.country_code.clone()).clicked() {
                                            self.category_selected = index
                                        }
                                        ui.end_row()
                                    }
                                }
                            });
                        });
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

        if trigger_download {
            let request = ehttp::Request::get(GEOSITE_URL);
            let download_store = self.geosite.clone();
            let _ctx = ctx.clone();
            *download_store.lock().unwrap() = Download::InProgress;
            ehttp::fetch(request, move |response| {
                let v = geodata::deserialize_geosite(&response.unwrap().bytes);
                let mut s = v.unwrap().entry.clone();
                s.sort_by_key(|x: &geodata::GeoSite| x.country_code.clone());
                *download_store.lock().unwrap() = Download::Done(Ok(s));
                _ctx.request_repaint();
            });
        }
    }
}
