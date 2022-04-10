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
    let about = Paragraph {
        title: "About",
        content: cx.render(rsx!{
            p { "test" }
        })
    };

    let yeet = Paragraph {
        title: "yeet",
        content: cx.render(rsx!{
            p { "lmao" }
        })
    };
    cx.render(rsx!{
        about.render(&cx)
        yeet.render(&cx)
    })
}

fn main() {
    dioxus::web::launch(app);
}
