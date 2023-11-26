#![allow(non_snake_case)]

use dioxus::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryPageData {
    #[serde(flatten)]
    pub item: StoryItem,
    #[serde(default)]
    pub comments: Vec<Comment>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    /// there will be no by field if the comment was deleted
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub text: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    #[serde(default)]
    pub sub_comments: Vec<Comment>,
    pub r#type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryItem {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub descendants: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    pub r#type: String,
}


fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    render! {
        StoryListing {

        }
    }
}

#[inline_props]
fn StoryListing(cx: Scope, story: StoryItem) -> Element {
    let title = "title";
    let by = "author";
    let score = 0;
    let time = chrono::Utc::now();
    let comments = "comments";

    render! {
        div {
            padding: ".5rem",
            background: "#242424",
            color: "#eeee",
            border_radius: "9px",
            border: "none",
            box_shadow: "0 1rem 1rem 0 rgba(0, 0, 0, .2)",
            font_family: "monospace",
            font_weight: "600",
            "{title} by {by} ({score}) {time} {comments}"
        }
    }
}
