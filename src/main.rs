mod about;
mod book;
mod bike_shed;

use book::*;
use about::about;
use bike_shed::*;

use convert_case::{Case, Casing};
use dioxus::prelude::*;

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

#[derive(Props)]
struct WebsiteProp<'a> {
    simple_pages: Vec<PageProp<'a>>,
    books: Vec<BookProp<'a>>,
}

#[allow(non_snake_case)]
fn Website<'a>(cx: Scope<'a, WebsiteProp<'a>>) -> Element<'a> {
    cx.render(rsx! {
        nav {
            ul {
                cx.props.simple_pages.iter().map(|page| {
                    let route = SiteRoute::main(page.title).build();
                    cx.render(rsx! {
                        li {
                            a {
                                href: "{route}",
                                "{page.title}"
                            }
                        }
                    })
                })
                cx.props.books.iter().map(|book| {
                    let route = SiteRoute::main(book.title).build();
                    cx.render(rsx! {
                        li {
                            a {
                                href: "{route}",
                                "{book.title}"
                            }
                        }
                    })
                })
            }
        }

        Router {
            cx.props.simple_pages.iter().map(|page| {
                let route = SiteRoute::main(page.title).build();
                cx.render(rsx! {
                    Route {
                        to: "{route}",
                        page.render(cx)
                    }
                })
            })
        }

        cx.props.books.iter().map(|book| book.routes(cx))
    })
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Website {
            simple_pages: vec![
                about(cx)
            ],
            books: vec![
                bike_shed()
            ]
        }
    })
}

fn main() {
    dioxus::web::launch(app);
}
