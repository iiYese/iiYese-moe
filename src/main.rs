mod about;
mod book;

use crate::book::*;

use convert_case::{Case, Casing};
use dioxus::prelude::*;

pub fn to_url_case(parent: &str, title: &str) -> String {
    format!("{}/{}", parent, title).to_case(Case::Kebab)
}

enum Section<'a> {
    Page(PageProp<'a>),
    Book(BookProp<'a>),
}

struct WebsiteProp<'a> {
    sections: Vec<Section<'a>>,
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Page {
            title: "Dioxus",
            Paragraph {
                title: "Yaa",
                p { "yeet" }
            }

            Paragraph {
                title: "Ayy",
                p{ "lmao" }
            }
        }
    })
}

fn main() {
    dioxus::web::launch(app);
}
