#![no_std]
extern crate alloc;
use core::fmt::write;

use alloc::string::String;

use gloo::events::EventListener;
use gloo::utils::document;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

use crate::wordlist::WORDS;

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

pub fn regenerate() {
    use core::fmt::Write;
    use rand::prelude::*;

    let mut rng = rand::rngs::OsRng;

    let document = document();

    let el_words = document.get_element_by_id("words").unwrap();
    let el_passphrase = document.get_element_by_id("passphrase").unwrap();
    let el_strength = document.get_element_by_id("strength").unwrap();

    let el_words = el_words.dyn_ref::<HtmlInputElement>().unwrap();

    let word_count = el_words.value().parse::<f64>().unwrap_or_default() as u8;

    // The longest word in our wordlist is 9 bytes.
    // With `N` words we have `N-1` spaces between them.
    // Therefore we only need to reserve a capacity of
    // `(9*N) + (N-1)` or `10*N - 1` to avoid a reallocation.
    // We also need to reserve 7 bytes for the paragraph tags,
    // so that brings us to `10*N + 6`.
    // For simplicity's sake I round to `10*(N + 1)`
    let cap = usize::from(10 * (word_count as u16 + 1));
    let mut passphrase = String::with_capacity(cap);
    passphrase.push_str("<p>");

    for i in 0..word_count {
        if i != 0 {
            passphrase.push(' ');
        }
        let word = *wordlist::WORDS.choose(&mut rng).unwrap();
        passphrase.push_str(word);
    }
    passphrase.push_str("</p>");
    el_passphrase.set_inner_html(&passphrase);

    let weak = Seconds::cheap_hw(word_count);
    let strong = Seconds::good_hw(word_count);

    let mut desc = String::new();
    writeln!(&mut desc, "<p>An adversary on a budget* could crack this password in <b>{weak}</b>.</p>").unwrap();
    writeln!(&mut desc, "<p>A well-funded adversary** could crack this password in <b>{strong}</b>.</p>").unwrap();
    el_strength.set_inner_html(&desc)
    /*
    adversary
    enemy
    opponent
    hacker
     */
}

#[derive(Clone, Copy)]
struct Seconds(f64);

impl Seconds {
    fn cheap_hw(words: u8) -> Self {
        const WORDS_LEN: f64 = 7776.0;
        const HASHES_PER_SEC: f64 = 20.6e9;

        if words == 0 {
            Self(0.0)
        } else {
            let guesses = 0.5 * WORDS_LEN.powf(words.into());
            log::debug!("{guesses}");
            Self(guesses / HASHES_PER_SEC)
        }
    }

    fn good_hw(words: u8) -> Self {
        let Self(s) = Self::cheap_hw(words);
        Self(s / 100.0)
    }
}

impl core::fmt::Display for Seconds {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self(s) = *self;

        if s < 2.0 {
            return write!(f, "1 second");
        }

        if s < 60.0 {
            let s = s.max(2.0).floor() as u8;
            return write!(f, "{s} seconds");
        }
        // 60 seconds in a minute
        let min = s / 60.0;

        if min <= 59.0 {
            let min = min.max(1.0).floor() as u8;
            return write!(f, "{min} minutes");
        }
        // 60 minutes in an hour
        let h = min / 60.0;

        if h <= 23.0 {
            let h = h.max(1.0).floor() as u8;
            return write!(f, "{h} hours");
        }
        // 24 hours in a day
        let d = h / 24.0;

        if d <= 30.0 {
            let d = d.max(1.0).floor() as u8;
            return write!(f, "{d} days");
        }
        // Approx 30.416... days in a month
        let mon = d / 30.41666666666666;

        if mon <= 11.0 {
            let mon = mon.max(1.0).floor() as u8;
            return write!(f, "{mon} months");
        }
        // 365.24... days in a year
        let y = d / 365.242374;

        if y <= 999.0 {
            let y = y.max(1.0).floor() as u8;
            return write!(f, "{y} years");
        }
        // Thousands of years
        let ty = y / 1_000.0;

        if ty <= 999.0 {
            let ty = ty.max(1.0).floor() as u8;
            return write!(f, "{ty} thousand years");
        }
        // Millions of years
        let my = ty / 1_000.0;

        if my <= 999.0 {
            let my = my.max(1.0).floor() as u8;
            return write!(f, "{my} million years");
        }
        // Billions of years
        let by = my / 1_000.0;

        if by <= 999.0 {
            let by = by.max(1.0).floor() as u8;
            return write!(f, "{by} billion years");
        }
        // Trillions of years
        let trly = by / 1_000.0;

        let trly = trly.max(1.0).floor() as u64;
        write!(f, "{trly} trillion years")
    }
}