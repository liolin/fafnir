table! {
    contacts (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
    }
}

table! {
    mails (id) {
        id -> Int4,
        label -> Varchar,
        address -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    contacts,
    mails,
);
