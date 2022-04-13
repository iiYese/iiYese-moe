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
                    thead {
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
                    }
                    thead {
                        tr {
                            th { h3 { "Python" } }
                            td { 
                                code { "Fast API" }
                                code { "Django" }
                            }
                        }
                    }
                    thead {
                        tr {
                            th { h3 { "C++" } }
                            td {
                                code { "SFML" }
                                code { "Boost" } 
                            }
                        }
                    }
                    thead {
                        tr {
                            th { h3 { "Clojure" } }
                            td {
                                code { "reagent" }
                                code { "shadowcljs" } 
                            }
                        }
                    }
                    thead {
                        tr {
                            th { h3 { "C" } }
                            td {
                                code { "glibc" }
                            }
                        }
                    }
                    thead {
                        tr {
                            th { h3 { "More" } }
                            td {
                                b { "WASM" }
                                b { "Haskell" }
                                b { "Java" }
                                b { "C#" }
                                b { "PrimeReact" }
                            }
                        }
                    }
                }
            }

            Paragraph {
                title: "Projects",

                h3 { "OMFG - Open Modding Framework (for) Games" } 
                br {}
            }
        })
    }
}
