table! {
    boards (id) {
        id -> Int4,
        path -> Text,
        name -> Text,
    }
}

table! {
    posts (id) {
        id -> Int4,
        board -> Text,
        text -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    boards,
    posts,
);
