// @generated automatically by Diesel CLI.

diesel::table! {
    frequencies (id) {
        id -> Unsigned<Bigint>,
        label -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Text,
    }
}

diesel::table! {
    vehicle_types (id) {
        id -> Unsigned<Bigint>,
        label -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(frequencies, users, vehicle_types,);
