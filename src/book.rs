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
    pub title: &'a str,
    pub children: Element<'a>,
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
    pub title: &'a str,
    pub description: &'a str,
    pub pages: Vec<PageProp<'a>>,
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

impl<'a> BookProp<'a> {
    fn menu<T>(&'a self, cx: Scope<'a, T>) -> Element<'a> {
        let book_route = SiteRoute::main(self.title).build();
        let title = self.title;

        cx.render(rsx!{
            aside {
                ul {
                    li {
                        a {
                            href: "{book_route}",
                            "{title}"
                        }
                    }
                    self.chapters.iter().map(|chapter| {
                        let route = SiteRoute::main(self.title)
                            .sub(chapter.title)
                            .build();

                        cx.render(rsx!{
                            li {
                                a {
                                    href: "{route}",
                                    "{chapter.title}"
                                }
                                ul {
                                    chapter.pages.iter().map(|page| {
                                        let route = SiteRoute::main(self.title)
                                            .sub(chapter.title)
                                            .sub(page.title)
                                            .build();

                                        cx.render(rsx!{
                                            li {
                                                a {
                                                    href: "{route}",
                                                    "{page.title}"
                                                }
                                            }
                                        })
                                    })
                                }
                            }
                        })
                    })
                }
            }
        })
    }

    fn routes<T>(&'a self, cx: Scope<'a, T>) -> Element<'a> {
        let book_route = SiteRoute::main(self.title).build();
        let title = self.title;
        let description = self.description;

        cx.render(rsx!{
            Router {
                Route {
                    to: "{book_route}",
                    
                    self.menu(cx)
                    
                    Page {
                        title: "{title}",
                        "{description}" 
                    }
                }
                self.chapters.iter().map(|chapter| {
                    let title = chapter.title;
                    let description = chapter.description;
                    let route = SiteRoute::main(self.title)
                        .sub(chapter.title)
                        .build();

                    cx.render(rsx!{
                        Route {
                            to: "{route}",
                            
                            self.menu(cx)

                            Page {
                                title: "{title}",
                                "{description}"
                            }
                        }
                        chapter.pages.iter().map(|page| {
                            let route = SiteRoute::main(self.title)
                                .sub(chapter.title)
                                .sub(page.title)
                                .build();

                            cx.render(rsx!{
                                Route {
                                    to: "{route}",
                                    
                                    self.menu(cx)

                                    page.render(cx)
                                }
                            })
                        })
                    })
                })
            }
        })
    }
}

pub fn Book<'a>(cx: Scope<'a, BookProp<'a>>) -> Element<'a> {
    cx.props.routes(cx)
}
