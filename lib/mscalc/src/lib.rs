#![no_std]
extern crate alloc;
use alloc::format;

use gloo::events::EventListener;
use gloo::utils::document;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

// Characteristic impedance of free space
//
// Refer to: https://en.wikipedia.org/wiki/Impedance_of_free_space
const Z_F: f64 = 376.73031366857;

#[wasm_bindgen(start)]
pub fn init() {
    use log::Level;
    
    const WIDTH: &str = "127";
    const HEIGHT: &str = "127";
    const ER: &str = "9.6";

    let () = console_error_panic_hook::set_once();
    let () = console_log::init_with_level(Level::Debug).unwrap();

    let document = document();

    let width = document.get_element_by_id("width").unwrap();
    let height = document.get_element_by_id("height").unwrap();
    let er = document.get_element_by_id("er").unwrap();

    width
        .dyn_ref::<HtmlInputElement>()
        .unwrap()
        .set_value(WIDTH);
    height
        .dyn_ref::<HtmlInputElement>()
        .unwrap()
        .set_value(HEIGHT);
    er.dyn_ref::<HtmlInputElement>().unwrap().set_value(ER);

    let on_change = move |_: &_| show_z();

    EventListener::new(&width, "change", on_change).forget();
    EventListener::new(&height, "change", on_change).forget();
    EventListener::new(&er, "change", on_change).forget();
    on_change(&web_sys::Event::new("").unwrap());
}

#[wasm_bindgen]
pub fn show_z() {
    let document = document();

    let elem_width = document.get_element_by_id("width").unwrap();
    let elem_height = document.get_element_by_id("height").unwrap();
    let elem_er = document.get_element_by_id("er").unwrap();

    let elem_width = elem_width.dyn_ref::<HtmlInputElement>().unwrap();
    let elem_height = elem_height.dyn_ref::<HtmlInputElement>().unwrap();
    let elem_er = elem_er.dyn_ref::<HtmlInputElement>().unwrap();

    let w = elem_width.value().parse().unwrap_or_default();
    let h = elem_height.value().parse().unwrap_or_default();
    let er = elem_er.value().parse().unwrap_or_default();

    let w = if w < 1.0 {
        elem_width.set_value("1");
        1.0
    } else if w > 1000.0 {
        elem_width.set_value("1000");
        1000.0
    } else {
        w
    };
    let h = if h < 1.0 {
        elem_height.set_value("1");
        1.0
    } else if h > 1000.0 {
        elem_height.set_value("1000");
        1000.0
    } else {
        h
    };
    let er = if er < 1.0 {
        elem_er.set_value("1");
        1.0
    } else if er > 30.0 {
        elem_er.set_value("30");
        30.0
    } else {
        er
    };
    let z = z(w, h, er);
    let z_result = document.get_element_by_id("z_result").unwrap();
    z_result.set_inner_html(&format!("{z:.2} Ω"));
}

// Algorithm from Petersson, M. “Microstrip Solutions for Innovative Microwave Feed Systems.” (2001)
fn z(w: f64, h: f64, er: f64) -> f64 {
    const TWO_PI: f64 = 2.0 * core::f64::consts::PI;

    let wh = w / h;

    let e1 = 0.5 * (er + 1.0);
    let e2 = 0.5 * (er - 1.0);

    let x = 1.0 / (1.0 + 12.0 / wh).sqrt();

    if wh > 1.0 {
        let e_eff = e1 + e2 * x;

        Z_F / (e_eff.sqrt() * (1.393 + wh + 2.0 / 3.0 * (wh + 1.444).ln()))
    } else {
        let e_eff = e1 + e2 * (x + 0.04 * (1.0 - wh).powi(2));
        Z_F / (TWO_PI * e_eff.sqrt()) * (8.0 / wh + 0.25 * wh).ln()
    }
}
