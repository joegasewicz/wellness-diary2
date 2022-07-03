table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    events (id) {
        id -> Int4,
        details -> Varchar,
        date -> Nullable<Timestamptz>,
        user_id -> Int8,
        category_id -> Nullable<Int4>,
    }
}

table! {
    symptom_types (id) {
        id -> Int4,
        name -> Varchar,
        user_id -> Int8,
    }
}

table! {
    symptoms (id) {
        id -> Int4,
        date -> Nullable<Timestamptz>,
        details -> Nullable<Varchar>,
        user_id -> Int8,
        symptom_types_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int8,
        email -> Varchar,
        password -> Varchar,
        name -> Nullable<Varchar>,
    }
}

joinable!(events -> categories (category_id));
joinable!(events -> users (user_id));
joinable!(symptom_types -> users (user_id));
joinable!(symptoms -> symptom_types (symptom_types_id));
joinable!(symptoms -> users (user_id));

allow_tables_to_appear_in_same_query!(
    categories,
    events,
    symptom_types,
    symptoms,
    users,
);
