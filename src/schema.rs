table! {
    characters (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    characters_stats (id) {
        id -> Int4,
        name -> Varchar,
        alignment -> Varchar,
        intelligence -> Int4,
        strengh -> Int4,
        speed -> Int4,
        durability -> Int4,
        power -> Int4,
        combat -> Int4,
        total -> Int4,
    }
}

table! {
    characters_to_comics (id) {
        id -> Int4,
        comics_id -> Int4,
        characters_id -> Int4,
    }
}

table! {
    comics (id) {
        id -> Int4,
        title -> Varchar,
        issuenumber -> Nullable<Float8>,
        description -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    characters,
    characters_stats,
    characters_to_comics,
    comics,
);
