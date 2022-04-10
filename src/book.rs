use convert_case::{Casing, Case};
use dioxus::prelude::*;
use crate::to_url_case;

pub struct Paragraph<'a> {
    pub title: &'a str,
    pub content: Element<'a>,
}

impl<'a> Paragraph<'a> {
    pub fn render(&self, cx: &Scope<'a>) -> Element<'a> {
        let link = self.title.to_case(Case::Kebab);
        let title = self.title;
        cx.render(rsx!{
            div {
                class: "paragraph",
                id: "{link}",
                a { 
                    href: "#{link}", 
                    div { 
                        class: "hr-sect", 
                        span { "{title}" } 
                    }
                }
                self.content.as_ref()
            }
        })
    }
}

pub struct Page<'a> {
    title: &'a str,
    content: Vec<Paragraph<'a>>,
}

impl<'a> Page<'a> {
    pub fn render(&self, cx: &Scope<'a>) -> Element<'a> {
        cx.render(rsx! {
            self.content.iter().map(|p| p.render(cx))
        })
    }
}

struct Chapter<'a> {
    title: &'a str,
    pages: Vec<Page<'a>>,
}

struct Book<'a> {
    title: &'a str,
    chapters: Vec<Chapter<'a>>,
}
