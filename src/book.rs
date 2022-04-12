#![allow(non_snake_case)]
use crate::to_url_case;
use convert_case::{Case, Casing};
use dioxus::prelude::*;

#[derive(Props)]
pub struct ParagraphProp<'a> {
    title: &'a str,
    children: Element<'a>,
}

pub fn Paragraph<'a>(cx: Scope<'a, ParagraphProp<'a>>) -> Element<'a> {
    let link = cx.props.title.to_case(Case::Kebab);
    cx.render(rsx! {
        div {
            class: "paragraph",
            id: "{link}",
            a {
                href: "#{link}",
                div {
                    class: "hr-sect",
                    span { "{cx.props.title}" }
                }
            }
            cx.props.children.as_ref()
        }
    })
}

#[derive(Props)]
pub struct PageProp<'a> {
    title: &'a str,
    children: Element<'a>,
}

impl<'a> PageProp<'a> {
    fn render<T>(&'a self, cx: Scope<'a, T>) -> Element {
        let title = self.title;
        cx.render(rsx! {
            div {
                class: "page",
                h1 { "{title}" }
                self.children.as_ref()
            }
        })
    }
}

pub fn Page<'a>(cx: Scope<'a, PageProp<'a>>) -> Element<'a> {
    cx.props.render(cx)
}

pub struct Chapter<'a> {
    title: &'a str,
    decription: &'a str,
    pages: Vec<PageProp<'a>>,
}

#[derive(Props)]
pub struct BookProp<'a> {
    title: &'a str,
    description: &'a str,
    chapters: Vec<Chapter<'a>>,
}

struct SiteRoute(String);

impl SiteRoute {
    fn main(s: impl AsRef<str>) -> Self {
        Self("/".to_string() + &s.as_ref().to_case(Case::Kebab))
    }

    fn sub(self, s: impl AsRef<str>) -> Self {
        Self(self.0 + "/" + &s.as_ref().to_case(Case::Kebab))
    }

    fn build(self) -> String {
        self.0
    }
}

fn Book<'a>(cx: Scope<'a, BookProp<'a>>) -> Element<'a> {
    let book_route = SiteRoute::main(cx.props.title).build();
    let menu = cx.render(rsx!{
        ol {
            id: "menu",
            li {
                a {
                    href: "{book_route}",
                    "{cx.props.title}"
                }
                ol {
                    cx.props.chapters.iter().map(|chapter| {
                        let route = SiteRoute::main(cx.props.title)
                            .sub(chapter.title)
                            .build();

                        cx.render(rsx!{
                            li {
                                a {
                                    href: "{route}",
                                    "{chapter.title}"
                                }
                            }
                        })
                    })
                }
            }
        }
    });

    
    cx.render(rsx! {
        p {}
    })
}
