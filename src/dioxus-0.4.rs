#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(|cx| render! { "hello world!" })
}
