use crate::book::*;
use dioxus::prelude::*;

pub fn about(cx: Scope) -> PageProp {
    PageProp {
        title: "About",
        children: cx.render(rsx! {
            p { "I am a software developer. I primarily work with Native Applications,
                Systems Programs, UX, Games and Backend Web.
                Here are some of the things I've worked with, done or am doing."
            }

            br{}
            
            Paragraph {
                title: "Technologies",
                table {
                    class: "htable",
                    tr {
                        th { h3 { "Rust"} }
                        td { 
                            code { "Tokio" }
                            code { "Rocket" }
                            code { "Serde" }
                            code { "Desil" }
                            code { "Dioxus" }
                            code { "WGPU" } 
                            code { "Bevy" } 
                        }
                    }
                    tr {
                        th { h3 { "Python" } }
                        td { 
                            code { "Fast API" }
                            code { "Django" }
                        }
                    }
                    tr {
                        th { h3 { "C++" } }
                        td {
                            code { "SFML" }
                            code { "Boost" } 
                        }
                    }
                    tr {
                        th { h3 { "Clojure" } }
                        td {
                            code { "Reagent" }
                            code { "Reitit" }
                            code { "Shadow-cljs" } 
                        }
                    }
                    tr {
                        th { h3 { "C" } }
                        td {
                            code { "glibc" }
                        }
                    }
                    tr {
                        th { h3 { "More" } }
                        td {
                            b { "WASM" }
                            b { "FFI" }
                            b { "Haskell" }
                            b { "Java" }
                            b { "C#" }
                            b { "PrimeReact" }
                        }
                    }
                }
            }

            Paragraph {
                title: "Projects",

                h3 { "OMFG - Open Modding Framework (for) Games" } 
                p { 
                    "OMFG is a companion CLI for games that rely heavily on 
                    community created content to help them implement 
                    in editor modding tools to let users curate
                    the content they generate more easily." 
                }
                p { 
                    "Most games serialize their level data to text formats.
                    OMFG leverages this to achieve language agnosticism." 
                }
                p { 
                    "When I looked at different text serialization formats 
                    I noticed that they tend to come in families.
                    This was used to achieve serialization format agnosticism." 
                }
                p {
                    "OMFG is written using the Rust ecosystem."
                }
            }
        })
    }
}
