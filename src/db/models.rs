/**
 * This file contains the database models for diesel
 */

use diesel::prelude::*;

/// Struct to represent a guild member
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub ingame_name: String,
    pub discord_tag: String,
    pub temp_ping: bool,
    pub perma_ping: bool,
}


/// Struct to represent an ingame guild
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::guilds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Guild {
    pub id: i32,
    pub name: String,
}


/// Struct to represent an ingame item
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Item {
    pub name: String,
    pub price: i32,
    pub raw_item: bool,
}


/// Struct to represent an ingame crating reciep
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::recieps)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Reciep {
    pub id: i32,
    pub resulting_item_id: i32,
    pub produced_amount: i32,
    pub duration: i32,
}


/// Struct to represent a part of a reciep
#[derive(Queryable, Selectable)]
#[diesel(belongs_to(Reciep))]
#[diesel(table_name = crate::db::schema::reciep_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReciepItem {
    pub reciep_id: i32,
    pub item_id: i32,
    pub amount: i32,
}


/// Struct to represent an ingame event
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Event {
    pub id: i32,
    pub active: bool,
    pub year: i32,
    pub week: i32,
    pub first_item_id: i32,
    pub second_item_id: i32,
    pub third_item_id: i32,
    pub fourth_item_id: i32,
    pub first_item_base_amount: i32,
    pub second_item_base_amount: i32,
    pub third_item_base_amount: i32,
    pub fourth_item_base_amount: i32,
}


/// Struct to represent results of an event for a specific guilds
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::event_results)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EventResults {
    pub event_id: i32,
    pub guild_id: i32,
    pub overall_donations: i32,
    pub active_players: i32,
    pub reached_round: i32,
}