#![no_std]
extern crate alloc;
use alloc::format;

use gloo::utils::document;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

// Characteristic impedance of free space
//
// Refer to: https://en.wikipedia.org/wiki/Impedance_of_free_space
const Z_F: f64 = 376.73031366857;

#[wasm_bindgen]
pub fn show_z(w: f64, h: f64, er: f64) {
    let document = document();

    let set_elem = |elem, s| {
        document
            .get_element_by_id(elem)
            .unwrap()
            .dyn_ref::<HtmlInputElement>()
            .unwrap()
            .set_value(s)
    };
    let w = if w < 1.0 {
        set_elem("width", "1");
        1.0
    } else if w > 1000.0 {
        set_elem("width", "1000");
        1000.0
    } else {
        w
    };
    let h = if h < 1.0 {
        set_elem("height", "1");
        1.0
    } else if h > 1000.0 {
        set_elem("height", "1000");
        1000.0
    } else {
        h
    };
    let er = if er < 1.0 {
        set_elem("er", "1");
        1.0
    } else if er > 30.0 {
        set_elem("er", "30");
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
