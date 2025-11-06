#![windows_subsystem = "windows"]

use dioxus::prelude::*;
use dioxus::desktop::muda::*;

mod app;
mod define;
mod json;
mod ui;

fn main() {
    let mut json = json::Json::new();
    json.load();
    let pos = dioxus::desktop::tao::dpi::PhysicalPosition::new(json.wi.pos_x, json.wi.pos_y);
    let size = dioxus::desktop::tao::dpi::PhysicalSize::new(json.wi.width, json.wi.height);
    let ico = app::load_icon_from_url(define::common::TOOLICONURL);
    let wb = dioxus::desktop::WindowBuilder::new()
        .with_window_icon(ico)
        .with_title(define::common::TOOLNAME)
        .with_position(pos)
        .with_inner_size(size);
    let config = dioxus::desktop::Config::new().with_menu(Menu::new()).with_window(wb);
    LaunchBuilder::new().with_cfg(config).launch(ui::App);
}
