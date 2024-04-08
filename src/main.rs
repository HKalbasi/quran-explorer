#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
use quran::QuranSubset;

use crate::quran::QURAN;

mod quran;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home,
    #[route("/aya/:sura/:aya")]
    Aya { sura: usize, aya: usize },
    #[route("/sura/:sura")]
    Sura { sura: usize },
    #[route("/search/:query")]
    Search { query: String },
}

fn normalize(x: &str) -> String {
    x.chars()
        .filter(|&c| !(1614..=1618).contains(&(c as u32)))
        .collect()
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn QuranSubsetViewer(subset: QuranSubset) -> Element {
    rsx! {
        for (sura_index, sura, ayas) in subset.sura_iter() {
            h1 {
                "{sura.name}"
            }
            br {}
            for &aya in ayas {
                Link { to: Route::Aya { sura: sura_index, aya: aya + 1 }, "{aya + 1}" }
                br {}
                "{sura.aya[aya]}"
                br {}
            }
            br{}
        }
    }
}

#[component]
fn Search(query: String) -> Element {
    let nav = navigator();

    let x = QURAN.filter(|_, _, text| normalize(text).contains(&query));

    rsx! {
        input {
            // we tell the component what to render
            value: "{query}",
            // and what to do when the value changes
            oninput: move |event| { nav.replace(Route::Search { query: event.value() }); }
        }
        br {}
        QuranSubsetViewer { subset: x }
    }
}

#[component]
fn Sura(sura: usize) -> Element {
    let Some(sura_data) = QURAN.sura.get(sura - 1) else {
        return rsx! {
            "Not found {sura}"
        };
    };
    rsx! {
        div {
            font_size: "20px",
            "{sura_data.name}"
        }
        br {}
        if let Some(bismillah) = &sura_data.bismillah {
            "{bismillah}"
            br {}
        }
        for (aya_index, aya) in sura_data.aya.iter().enumerate() {
            div {
                font_size: "40px",
                "{aya}"
                Link { to: Route::Aya { sura, aya: aya_index + 1 }, "{aya_index + 1}" }
            }
            br {}
        }
        Link { to: Route::Sura { sura: sura - 1 }, "Previous sura" }
        br {}
        Link { to: Route::Sura { sura: sura + 1 }, "Next sura" }
    }
}

#[component]
fn Aya(sura: usize, aya: usize) -> Element {
    let Some(text) = QURAN.get_aya(sura, aya) else {
        return rsx! {
            "Not found {sura} {aya}"
        };
    };
    let sura_name = &QURAN.sura[sura - 1].name;
    rsx! {
        div {
            font_size: "20px",
            "{sura_name} {aya}"
        }
        div {
            font_size: "40px",
            height: "200px",
            "{text}"
        }
        br {}
        Link { to: Route::Aya { sura, aya: aya - 1 }, "Previous aya" }
        br {}
        Link { to: Route::Aya { sura, aya: aya + 1 }, "Next aya" }
        br {}
        Link { to: Route::Sura { sura }, "Whole sura" }
        br {}
        Link { to: Route::Aya { sura: sura - 1, aya }, "Previous sura" }
        br {}
        Link { to: Route::Aya { sura: sura + 1, aya }, "Next sura" }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            Link { to: Route::Search { query: "".to_string() }, "Search" }
            br {}
            for (i, sura) in QURAN.sura.iter().enumerate() {
                Link { to: Route::Sura { sura: i + 1 }, "{sura.name}" }
                " "
            }
        }
    }
}
