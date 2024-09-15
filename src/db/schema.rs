use diesel::{allow_tables_to_appear_in_same_query, joinable};

diesel::table! {
    users (id) {
        id -> Integer,
        ingame_name -> Text,
        discord_tag -> Text,

        /// Whether the player should appear in the list of inactive players. Is cleared automatically after each event.
        temp_ping -> Bool,

        /// Whether the player should appear in the list of inactive players. Is never cleared automatically
        perma_ping -> Bool,
    }
}

diesel::table! {
    guilds (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    items (name) {
        name -> Text,
        price -> Integer,

        /// Whether the item is obtainable through mines
        raw_item -> Bool,
    }
}

/*
 * Table that holds the recieps and their resulting item
 */
diesel::table! {
    recieps (id) {
        id -> Integer,

        /// ID of the item that is produced
        resulting_item_id -> Integer,

        /// Amount that is produced
        produced_amount -> Integer,

        /// Time (in sec) needed to complete one production process
        duration -> Integer,
    }
}

/*
 * Table that holds items needed for a reciep identified by the reciep_id
 */
diesel::table! {
    reciep_items (reciep_id, item_id) {
        /// ID of reciept this item is part of
        reciep_id -> Integer,

        item_id -> Integer,
        
        /// Needed amount for one production process
        amount -> Integer,
    }
}

joinable!(reciep_items -> recieps (reciep_id));
allow_tables_to_appear_in_same_query!(reciep_items, recieps);


/*
 * Table that holds information about an event
 */
diesel::table! {
    events (id) {
        id -> Integer,

        /// Whether the event is currently running
        active -> Bool,

        year -> Integer,
        week -> Integer,

        first_item_id -> Integer,
        first_item_base_amount -> Integer,

        second_item_id -> Integer,
        second_item_base_amount -> Integer,
        
        third_item_id -> Integer,
        third_item_base_amount -> Integer,
        
        fourth_item_id -> Integer,
        fourth_item_base_amount -> Integer,
    }
}


/*
 * Table that holds event result information for the event with matching event_id 
 */
diesel::table! {
    event_results (event_id, guild_id) {
        event_id -> Integer,
        guild_id -> Integer,
        overall_donations -> Integer,
        active_players -> Integer,
        reached_round -> Integer,
    }
}

joinable!(event_results -> events (event_id));
joinable!(event_results -> guilds (guild_id));
allow_tables_to_appear_in_same_query!(event_results, events);
allow_tables_to_appear_in_same_query!(event_results, guilds);