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
    timings (id) {
        id -> Unsigned<Bigint>,
        date -> Nullable<Date>,
        time -> Time,
        id_day -> Nullable<Unsigned<Bigint>>,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        firstname -> Text,
        lastname -> Text,
        #[max_length = 255]
        mail -> Varchar,
        passwd -> Text,
        token -> Nullable<Text>,
        token_gentime -> Nullable<Datetime>,
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

diesel::table! {
    vehicles (id) {
        id -> Unsigned<Bigint>,
        name -> Text,
        year -> Unsigned<Smallint>,
        nb_doors -> Unsigned<Tinyint>,
        nb_seats -> Unsigned<Tinyint>,
        trunk_size_L -> Unsigned<Smallint>,
        pic -> Nullable<Blob>,
        id_user -> Unsigned<Bigint>,
        id_type -> Unsigned<Bigint>,
    }
}

diesel::joinable!(timings -> days (id_day));
diesel::joinable!(vehicles -> vehicle_types (id_type));

diesel::allow_tables_to_appear_in_same_query!(
    days,
    frequencies,
    road_types,
    timings,
    users,
    vehicle_types,
    vehicles,
);
