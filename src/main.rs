use dioxus::prelude::*;

fn app(cx: Scope) -> Element {
    cx.render(rsx!{
        div { "hello, wasm!" }
    })
}

fn main() {
    dioxus::web::launch(app);
}
