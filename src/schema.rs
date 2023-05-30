// @generated automatically by Diesel CLI.

diesel::table! {
    days (id) {
        id -> Unsigned<Bigint>,
        label -> Text,
    }
}

diesel::table! {
    frequencies (id) {
        id -> Unsigned<Bigint>,
        label -> Text,
    }
}

diesel::table! {
    road_types (id) {
        id -> Unsigned<Bigint>,
        label -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        firstname -> Text,
        lastname -> Text,
        mail -> Varchar,
        passwd -> Text,
        profile_pic -> Nullable<Blob>,
        smoker -> Bool,
        id_favorite_vehicle -> Nullable<Unsigned<Bigint>>,
    }
}

diesel::table! {
    vehicle_types (id) {
        id -> Unsigned<Bigint>,
        label -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(days, frequencies, road_types, users, vehicle_types,);
