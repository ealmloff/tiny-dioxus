#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch::launch_cfg(|| rsx! { "hello world!" }, Default::default())
}
