#![allow(non_snake_case)]
use crate::SiteRoute;
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
            br{}
            br{}
            br{}
        }
    })
}

#[derive(Props)]
pub struct PageProp<'a> {
    pub title: &'a str,
    pub children: Element<'a>,
}

impl<'a> PageProp<'a> {
    pub fn render<T>(&'a self, cx: Scope<'a, T>) -> Element {
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

pub struct Book<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub chapters: Vec<Chapter<'a>>,
}

impl<'a> Book<'a> {
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

    pub fn routes<T>(&'a self, cx: Scope<'a, T>) -> Element<'a> {
        let book_route = SiteRoute::main(self.title).build();
        let title = self.title;
        let description = self.description;

        cx.render(rsx!{
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
        })
    }
}
