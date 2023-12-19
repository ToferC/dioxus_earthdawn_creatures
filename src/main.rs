#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use dioxus_demo::models::Creature;
use dioxus_fullstack::prelude::*;
use serde::Deserialize;
use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::PatchOp;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealdb::sql::Thing;
use once_cell::sync::Lazy;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[derive(Debug, Deserialize)]
struct Record {
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {

    DB.connect::<Ws>("127.0.0.1:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    }).await?;

    DB.use_ns("creatures").use_db("creatures").await?;

    let created: Vec<Record> = DB
        .create("creature")
        .content(Creature::default())
        .await?;

    dbg!(created);

    let updated: Option<Record> = DB
        .update(("creature", "ghoul"))
        .await?;

    dbg!(updated);

    let creature: Option<Creature> = DB
        .select(("creature", "zgzecnypuqlztg8fke9i"))
        .await?;

    dbg!(creature);

    let update: Option<Creature> = DB
        .update(("creature", "zgzecnypuqlztg8fke9i"))
        .patch(PatchOp::replace("/creature_name", "Zombie"))
        .patch(PatchOp::replace("/dex", 14))
        .await?;

    dbg!(update);

    LaunchBuilder::new(App).launch();

    Ok(())
}

pub fn App(cx: Scope) -> Element {
    render! {
        CreatureListing{}
    }
}

pub fn CreatureListing(cx: Scope) -> Element {
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
