#![allow(non_snake_case)]
use convert_case::{Casing, Case};
use dioxus::prelude::*;
use crate::to_url_case;

#[derive(Props)]
pub struct ParagraphProp<'a> {
    title: &'a str,
    content: Element<'a>,
}

pub fn Paragraph<'a>(cx: Scope<'a, ParagraphProp<'a>>) -> Element {
    let link = cx.props.title.to_case(Case::Kebab);
    cx.render(rsx!{
        div {
            id: "{link}",
            style: "padding-bottom: 3rem;",
            a { 
                href: "#{link}", 
                style: "text-decoration: none; font-size: 1.2rem; color: #333;", 
                div {
                    style: "
                        width: 100%;
                        height: 20px;
                        border-bottom: 2px solid #333;
                        text-align: center
                    ",
                    span {
                        style: "padding: 0 10px;",
                        "[#]"
                    }
                }
            }
            cx.props.content.as_ref()
        }
    })
}


struct PageProp<'a> {
    title: &'a str,
    content: Vec<ParagraphProp<'a>>,
}

struct Chapter<'a> {
    title: &'a str,
    pages: Vec<PageProp<'a>>,
}

struct Book<'a> {
    title: &'a str,
    chapters: Vec<Chapter<'a>>,
}
