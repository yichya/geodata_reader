mod geodata;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};

enum Download<T> {
    None,
    InProgress,
    Done(ehttp::Result<T>),
}

#[cfg(not(target_arch = "wasm32"))]
const GEOIP_URL: &str =
    "https://github.com/Loyalsoldier/v2ray-rules-dat/releases/latest/download/geoip.dat";

#[cfg(not(target_arch = "wasm32"))]
const GEOSITE_URL: &str =
    "https://github.com/Loyalsoldier/v2ray-rules-dat/releases/latest/download/geosite.dat";

#[cfg(target_arch = "wasm32")]
const GEOIP_URL: &str = "geoip.dat";

#[cfg(target_arch = "wasm32")]
const GEOSITE_URL: &str = "geosite.dat";

pub struct GeoDataReader {
    // Example stuff:
    domain_search: String,
    category_search: String,
    category_selected: usize,
    ip_search: String,
    country_code_search: String,
    country_code_selected: usize,
    geoip: Arc<Mutex<Download<Vec<geodata::GeoIp>>>>,
    geosite: Arc<Mutex<Download<Vec<geodata::GeoSite>>>>,
}

impl Default for GeoDataReader {
    fn default() -> Self {
        Self {
            domain_search: "".to_owned(),
            category_search: "".to_owned(),
            category_selected: 0,
            ip_search: "".to_owned(),
            country_code_search: "".to_owned(),
            country_code_selected: 0,
            geoip: Arc::new(Mutex::new(Download::None)),
            geosite: Arc::new(Mutex::new(Download::None)),
        }
    }
}

impl GeoDataReader {
    /// Called once before the first frame.
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for GeoDataReader {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            domain_search: _,
            category_search: _,
            category_selected: _,
            ip_search: _,
            country_code_search: _,
            country_code_selected: _,
            geoip: _,
            geosite: _,
        } = self;

        let mut trigger_geoip_download = false;
        egui::Window::new("GeoIP")
            .default_width(600.0)
            .show(ctx, |ui| {
                let gm: &Download<Vec<geodata::GeoIp>> = &self.geoip.lock().unwrap();
                match gm {
                    Download::Done(r) => match r {
                        Ok(geoip) => {
                            egui::SidePanel::left("geoip_left_panel")
                                .default_width(150.0)
                                .show_inside(ui, |ui| {
                                    egui::TextEdit::singleline(&mut self.country_code_search)
                                        .hint_text("Search Country Code")
                                        .show(ui);
                                    egui::TextEdit::singleline(&mut self.ip_search)
                                        .hint_text("Search IP Address")
                                        .show(ui);
                                    ui.separator();
                                    egui::ScrollArea::vertical().show(ui, |ui| {
                                        for (index, element) in geoip.iter().enumerate() {
                                            if element.country_code.contains(
                                                self.country_code_search
                                                    .to_ascii_uppercase()
                                                    .as_str(),
                                            ) {
                                                for ele2 in element.cidr.iter() {
                                                    let mut country_code_match = false;
                                                    match ele2.to_ipnet() {
                                                        Ok(n) => {
                                                            match self.ip_search.parse::<IpAddr>() {
                                                                Ok(ip_search_parsed) => {
                                                                    if n.contains(&ip_search_parsed)
                                                                    {
                                                                        country_code_match = true;
                                                                    }
                                                                }
                                                                Err(_) => {
                                                                    if self.ip_search.is_empty() {
                                                                        country_code_match = true;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(_) => continue,
                                                    }
                                                    if country_code_match {
                                                        if ui
                                                            .link(element.country_code.clone())
                                                            .clicked()
                                                        {
                                                            self.country_code_selected = index
                                                        }
                                                        ui.end_row();
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    });
                                });
                            egui::TopBottomPanel::top("geoip_top_panel")
                                .resizable(false)
                                .show_inside(ui, |ui| {
                                    ui.label(format!(
                                        "{} Total: {}",
                                        &geoip[self.country_code_selected].country_code,
                                        &geoip[self.country_code_selected].cidr.len()
                                    ));
                                });
                            egui::CentralPanel::default().show_inside(ui, |ui| {
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    for ele2 in &geoip[self.country_code_selected].cidr {
                                        match ele2.to_ipnet() {
                                            Ok(ipnet) => match self.ip_search.parse::<IpAddr>() {
                                                Ok(ip_search_parsed) => {
                                                    if ipnet.contains(&ip_search_parsed) {
                                                        ui.label(ipnet.to_string());
                                                    }
                                                }
                                                Err(_) => {
                                                    if self.ip_search.is_empty() {
                                                        ui.label(ipnet.to_string());
                                                    }
                                                }
                                            },
                                            Err(_) => {
                                                continue;
                                            }
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
                        trigger_geoip_download = true;
                    }
                }
            });

        if trigger_geoip_download {
            let request = ehttp::Request::get(GEOIP_URL);
            let download_store = self.geoip.clone();
            let _ctx = ctx.clone();
            *download_store.lock().unwrap() = Download::InProgress;
            ehttp::fetch(request, move |response| {
                let v = geodata::deserialize_geoip(&response.unwrap().bytes);
                let mut s = v.unwrap().entry;
                s.sort_by_key(|x: &geodata::GeoIp| x.country_code.clone());
                *download_store.lock().unwrap() = Download::Done(Ok(s));
                _ctx.request_repaint();
            });
        }

        let mut trigger_geosite_download = false;
        egui::Window::new("GeoSite")
            .default_width(800.0)
            .show(ctx, |ui| {
                let gm: &Download<Vec<geodata::GeoSite>> = &self.geosite.lock().unwrap();
                match gm {
                    Download::Done(r) => match r {
                        Ok(geosite) => {
                            egui::SidePanel::left("geosite_left_panel")
                                .resizable(false)
                                .exact_width(300.0)
                                .show_inside(ui, |ui| {
                                    egui::TextEdit::singleline(&mut self.category_search)
                                        .hint_text("Search Category")
                                        .show(ui);
                                    egui::TextEdit::singleline(&mut self.domain_search)
                                        .hint_text("Search Domain")
                                        .show(ui);
                                    ui.separator();
                                    egui::ScrollArea::vertical().show(ui, |ui| {
                                        for (index, element) in geosite.iter().enumerate() {
                                            if element.country_code.contains(
                                                self.category_search.to_ascii_uppercase().as_str(),
                                            ) {
                                                for ele2 in element.domain.iter() {
                                                    if ele2
                                                        .value
                                                        .contains(self.domain_search.as_str())
                                                    {
                                                        if ui
                                                            .link(element.country_code.clone())
                                                            .clicked()
                                                        {
                                                            self.category_selected = index
                                                        }
                                                        ui.end_row();
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    });
                                });
                            egui::TopBottomPanel::top("geosite_top_panel")
                                .resizable(false)
                                .show_inside(ui, |ui| {
                                    ui.label(format!(
                                        "{} Total: {}",
                                        &geosite[self.category_selected].country_code,
                                        &geosite[self.category_selected].domain.len()
                                    ));
                                });
                            egui::CentralPanel::default().show_inside(ui, |ui| {
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    for ele2 in &geosite[self.category_selected].domain {
                                        if ele2.value.contains(self.domain_search.as_str()) {
                                            ui.label(&ele2.value);
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
                        trigger_geosite_download = true;
                    }
                }
            });

        if trigger_geosite_download {
            let request = ehttp::Request::get(GEOSITE_URL);
            let download_store = self.geosite.clone();
            let _ctx = ctx.clone();
            *download_store.lock().unwrap() = Download::InProgress;
            ehttp::fetch(request, move |response| {
                let v = geodata::deserialize_geosite(&response.unwrap().bytes);
                let mut s = v.unwrap().entry;
                s.sort_by_key(|x: &geodata::GeoSite| x.country_code.clone());
                *download_store.lock().unwrap() = Download::Done(Ok(s));
                _ctx.request_repaint();
            });
        }
    }
}
