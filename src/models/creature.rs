use chrono::NaiveDateTime;
use uuid::Uuid;

use serde::{Serialize, Deserialize};
use surrealdb::engine::remote::ws::Ws;



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Creature {
    pub id: Uuid,
    pub creature_name: String,
    pub found_in: Vec<Locales>,
    pub rarity: Rarity,
    pub circle_rank: u32,
    pub dex: u32,
    pub strength: u32,
    pub con: u32,
    pub per: u32,
    pub wil: u32,
    pub cha: u32,
    pub initiative: u32,
    pub pd: u32,
    pub md: u32,
    pub sd: u32,
    pub pa: u32,
    pub ma: u32,
    pub unconsciousness_rating: u32,
    pub death_rating: u32,
    pub wound: u32,
    pub knockdown: u32,
    pub actions: u32,
    pub recovery_rolls: u32,
    pub image_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}



#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Unique,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Locales {
    Jungle,
    Desert,
    Forest,
    Plains,
    Urban,
    Mountain,
    Cavern,
    Kaer,
    Any,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InsertableCreature {
    pub creature_name: String,
    pub found_in: Vec<Locales>,
    pub rarity: Rarity,
    pub circle_rank: u32,
    pub dex: u32,
    pub strength: u32,
    pub con: u32,
    pub per: u32,
    pub wil: u32,
    pub cha: u32,
    pub initiative: u32,
    pub pd: u32,
    pub md: u32,
    pub sd: u32,
    pub pa: u32,
    pub ma: u32,
    pub unconscious_rating: u32,
    pub death_rating: u32,
    pub wound: u32,
    pub knockdown: u32,
    pub actions: u32,
    pub recovery_rolls: u32,
    pub image_url: String,
}

impl InsertableCreature {
    pub fn new(
        creature_name: String,
        found_in: Vec<Locales>,
        rarity: Rarity,
        circle_rank: u32,
        dex: u32,
        strength: u32,
        con: u32,
        per: u32,
        wil: u32,
        cha: u32,
        initiative: u32,
        pd: u32,
        md: u32,
        sd: u32,
        pa: u32,
        ma: u32,
        unconscious_rating: u32,
        death_rating: u32,
        wound: u32,
        knockdown: u32,
        actions: u32,
        recovery_rolls: u32,
        image_url: String,
    ) -> Self {
        InsertableCreature {
            creature_name,
            found_in,
            rarity,
            circle_rank,
            dex,
            strength,
            con,
            per,
            wil,
            cha,
            initiative,
            pd,
            md,
            sd,
            pa,
            ma,
            unconscious_rating,
            death_rating,
            wound,
            knockdown,
            actions,
            recovery_rolls,
            image_url,
        }
    }
}