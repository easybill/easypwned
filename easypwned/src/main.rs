#[cfg(feature = "ssr")]
pub mod backend;

use std::fmt::format;
use std::ops::Deref;
use dioxus::prelude::*;

#[cfg(feature = "ssr")]
use backend::setup;

use dioxus_fullstack::prelude::*;
use log::{LevelFilter, info};

use serde::{Deserialize, Serialize};
use serde;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> ::anyhow::Result<(), ::anyhow::Error> {
    let config = ServeConfigBuilder::new(app,());
    // let config = config.assets_path("easypwned/dist/");
    setup(config).await
}

#[cfg(feature = "web")]
fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch_cfg(app, dioxus_web::Config::new().hydrate(true));
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "min-h-full",
            header {
                class: "bg-white shadow",
                div {
                    class: "mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8",
                    nav {
                        class: "flex py-2",
                        a {
                            class: "ml-0 mr-12",
                            img {
                                class: "w-36",
                                src: "/assets/easybill.svg"
                            }
                        }
                    }
                }
            }
            password_section {}
            div {
                section {
                    class: "mx-auto max-w-7xl py-6 sm:px-6 lg:px-8",
                    div {
                        class: "max-w-1/2",
                        "easybill ist eine cloudbasierte Rechnungssoftware, die sich durch eine einfache Anwendung, umfassende Funktionalität und der vielfältigen Anbindung durch Schnittstellen schon seit mehr als 15 Jahren am Markt behauptet. Aktuell haben wir mehr als 15.000 aktive Kunden und wir wachsen stetig weiter."
                    }
                    h3 {
                        class: "text-lg font-bold tracking-tight text-gray-900",
                        "Du bist auf der Suche nach neuen Herausforderungen?",
                    }
                    ul {
                        class: "mt-3 ml-4 list-disc list-inside",
                        li {
                            a {
                                href: "https://www.easybill.de/jobs/software-developer-php",
                                "Software Developer PHP (m/w/d)"
                            }
                        }
                        li {
                            a {
                                href: "https://www.easybill.de/jobs/software-developer-ruby-on-rails",
                                "Software Developer Ruby on Rails (m/w/d)"
                            }
                        }
                        li {
                            a {
                                href: "https://www.easybill.de/jobs/devops-engineer",
                                "DevOps Engineer (m/w/d)"
                            }
                        }
                    }
                }
            }
        }
    })
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde()]
pub struct PwnedState {
    pub hash: String,
    pub pw: String,
    pub secure: bool,
}

async fn fetch_pwned_state(password: String) -> Option<PwnedState> {
    // let's for now ignore two digit or less long passwords
    if password.len() < 3 {
        return None;
    }

    // todo: url ausm wasm_bindgen holen, da hab ich ja die url
    Some(reqwest::get(format!("http://localhost:3342/pw/{:?}", password))
        .await
        .unwrap()
        .json::<PwnedState>()
        .await
        .expect("Something went wrong"))
}

fn password_section(cx: Scope) -> Element {
    let password = use_state(cx, || String::from(""));

    let fetch_pwned_status = use_future(cx, (password.get(), ), |(password,)| async move {
        fetch_pwned_state(password).await
    });

    let pwned_state = match fetch_pwned_status.value() {
        Some(s) => s.clone(),
        None => None,
    };

    let bg_color = match pwned_state.clone() {
        Some(v) => if v.secure {
            String::from("bg-emerald-400")
        } else {
            String::from("bg-red-400")
        },
        None => String::from("bg-sky-400")
    };

    info!("{:?}", pwned_state);

    cx.render(rsx! {
        div {
            class: "{bg_color}",
            section {
                class: "mx-auto max-w-7xl py-6 sm:px-6 lg:px-8",
                div {
                    h1 {
                        class: "text-3xl font-bold tracking-tight text-gray-900",
                        "EasyPwned"
                    }
                    div {
                        class: "my-3",
                        "Would that the Argo had never winged its way to the land of Colchis through the dark-blue Symplegades!1 Would that the pine trees had never been felled in the glens of Mount Pelion and furnished oars for the hands of the heroes who at Pelias' command set forth in quest of the Golden Fleece! For then my lady Medea would not have sailed to the towers of Iolcus, her heart smitten with love for Jason, or persuaded the daughters of Pelias to kill their father and hence now be inhabiting this land of Corinth, <separated from her loved ones and country. At first, to be sure, she had, even in Corinth, a good life with her husband and children, an exile loved by the citizens to whose land she had come, and lending to Jason himself all her support. This it is that most rescues life from trouble, when a woman is not at variance with her husband."
                    }

                    if pwned_state.is_some() {
                        rsx! {
                            div {
                                class: "flex p-4 mb-4 text-sm text-blue-800 rounded-lg bg-blue-50",
                                div {
                                    span {
                                        class: "font-medium",
                                        "Ergebnis",
                                        ul {
                                            class: "mt-1.5 ml-4 list-disc list-inside",
                                            li {
                                                b {
                                                    "Password:"
                                                }
                                                " {pwned_state.clone().unwrap().pw}"
                                            }
                                            li {
                                                b {
                                                    "Hash:"
                                                }
                                                " {pwned_state.clone().unwrap().hash}"
                                            }
                                            li {
                                                b {
                                                    "pwned?:"
                                                }
                                                if pwned_state.clone().unwrap().secure {
                                                    "Nein"
                                                } else {
                                                    "Ja"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    class: "relative z-0 w-full mb-6 group",
                    label {
                        id: "password_label",
                        r#for: "password",
                        class: "block mb-2 text-sm font-medium text-gray-900",
                        "Passwort",
                    }
                    input {
                        id: "password",
                        class: "shadow-sm bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5",
                        r#type: "text",
                        placeholder: "Passwort",
                        value: "{password}",
                        oninput: move |event| password.set(event.value.clone()),
                    }
                }
            }
        },
    })
}
