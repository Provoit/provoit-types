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
    trip_road_types (id_trip, id_road_type) {
        id_trip -> Unsigned<Bigint>,
        id_road_type -> Unsigned<Bigint>,
    }
}

diesel::table! {
    trips (id) {
        id -> Unsigned<Bigint>,
        start -> Text,
        end -> Text,
        max_people -> Unsigned<Tinyint>,
        price -> Float,
        recurring -> Bool,
        id_frequency -> Nullable<Unsigned<Bigint>>,
        id_vehicle -> Unsigned<Bigint>,
        id_start_timing -> Unsigned<Bigint>,
        id_end_timing -> Unsigned<Bigint>,
    }
}

diesel::table! {
    user_trips (id_user, id_trip) {
        id_user -> Unsigned<Bigint>,
        id_trip -> Unsigned<Bigint>,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        firstname -> Text,
        lastname -> Text,
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
diesel::joinable!(trip_road_types -> road_types (id_road_type));
diesel::joinable!(trip_road_types -> trips (id_trip));
diesel::joinable!(trips -> frequencies (id_frequency));
diesel::joinable!(trips -> vehicles (id_vehicle));
diesel::joinable!(user_trips -> trips (id_trip));
diesel::joinable!(user_trips -> users (id_user));
diesel::joinable!(vehicles -> vehicle_types (id_type));

diesel::allow_tables_to_appear_in_same_query!(
    days,
    frequencies,
    road_types,
    timings,
    trip_road_types,
    trips,
    user_trips,
    users,
    vehicle_types,
    vehicles,
);
