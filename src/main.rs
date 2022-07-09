#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        Box::new(geodata_reader::MyApp::default()),
        options
    );
}
