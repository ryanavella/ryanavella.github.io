#![no_std]
extern crate alloc;
use alloc::string::String;

use gloo::events::EventListener;
use gloo::utils::document;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

mod wordlist;

#[wasm_bindgen(start)]
pub fn init() {
    use log::Level;

    let () = console_error_panic_hook::set_once();
    let () = console_log::init_with_level(Level::Debug).unwrap();

    let document = document();

    let el_words = document.get_element_by_id("words").unwrap();
    let el_regenerate = document.get_element_by_id("regenerate").unwrap();

    let on_change = move |_: &_| regenerate();

    EventListener::new(&el_words, "change", on_change).forget();
    EventListener::new(&el_regenerate, "click", on_change).forget();
    on_change(&web_sys::Event::new("").unwrap());
}

#[wasm_bindgen]
pub fn regenerate() {
    use rand::prelude::*;

    let mut rng = rand::rngs::OsRng;

    let document = document();

    let el_words = document.get_element_by_id("words").unwrap();
    let el_passphrase = document.get_element_by_id("passphrase").unwrap();

    let el_words = el_words.dyn_ref::<HtmlInputElement>().unwrap();

    let word_count = el_words.value().parse::<f64>().unwrap_or_default() as u8;

    // The longest word in our wordlist is 9 bytes.
    // With `N` words we have `N-1` spaces between them.
    // Therefore we only need to reserve a capacity of
    // `(9*N) + (N-1)` or `10*N - 1` to avoid a reallocation.
    // For simplicity's sake I just round this to `10*N`.
    let cap = usize::from(10 * word_count);
    let mut passphrase = String::with_capacity(cap);

    for i in 0..word_count {
        if i != 0 {
            passphrase.push(' ');
        }
        let word = *wordlist::WORDS.choose(&mut rng).unwrap();
        passphrase.push_str(word);
    }
    el_passphrase.set_inner_html(&passphrase);
}
