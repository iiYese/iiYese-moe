use crate::book::*;
use dioxus::prelude::*;

pub fn about(cx: Scope) -> PageProp {
    PageProp {
        title: "About",
        children: cx.render(rsx! {
            p { "I am a software developer. I primarily work with Native Applications,
                Systems Programs, UX, Games, Backend Web and XR.
                Here are some of the things I've worked with, done or am doing."
            }
            Paragraph {
                title: "Technologies",
                
                h3 { "Rust" }
                    code { "WGPU" } " - " code { "Bevy" } " - " code { "Tokio" } " - "
                    code { "Rocket" } " - " code { "Dioxus" }
                br {}
                
                p { "C++, C, Clojure, Haskell, Java" }
            }
        })
    }
}
