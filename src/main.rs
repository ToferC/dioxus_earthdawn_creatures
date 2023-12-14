#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

fn main() {
    LaunchBuilder::new(App).launch();
}

pub fn App(cx: Scope) -> Element {
    render! {
        StoryListing{}
    }
}

pub fn StoryListing(cx: Scope) -> Element {
    let title = "title";
    let by = "author";
    let score = 0;
    let time = chrono::Utc::now();
    let comments = "comments";

    render! {
        div {
            padding: "0.5rem",
            position: "relative",
            "{title} by {by} ({score}) {time} {comments}"
        }
    }
}
