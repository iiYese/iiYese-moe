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
    let chapters = vec![
        Chapter {
            title: "About",
            description: "About Rust",
            pages: vec![
                PageProp {
                    title: "About things",
                    children: cx.render(rsx!(p { "about" }))
                },
            ],
        },
        Chapter {
            title: "Book things",
            description: "Book Rust",
            pages: vec![
                PageProp {
                    title: "Book",
                    children: cx.render(rsx!(p { "book" }))
                },
            ],
        },
    ];
    cx.render(rsx! {
        Book {
            title: "Rust",
            description: "Rust is a systems programming language that is used to build software",
            chapters: chapters
        }
        Router {
            Route {
                to: "test"
                p { "test" }
            }
        }
    })
}

fn main() {
    dioxus::web::launch(app);
}
