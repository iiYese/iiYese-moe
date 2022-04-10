mod about;
mod blog;
mod book;

use crate::book::*;

use dioxus::prelude::*;
use convert_case::{Case, Casing};

pub fn to_url_case(parent: &str, title: &str) -> String {
    format!("{}/{}", parent, title).to_case(Case::Kebab)
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!{
        Paragraph {
            title: "About",
            content: cx.render(rsx!{
                p { "test" }
            })
        }
        Paragraph {
            title: "yeet",
            content: cx.render(rsx!{
                p { "lmao" }
            })
        }
    })
}

fn main() {
    dioxus::web::launch(app);
}
