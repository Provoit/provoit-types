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
        name -> Text,
    }
}

diesel::table! {
    vehicle_types (id) {
        id -> Unsigned<Bigint>,
        label -> Text,
    }
}

diesel::joinable!(timings -> days (id_day));

diesel::allow_tables_to_appear_in_same_query!(
    days,
    frequencies,
    road_types,
    timings,
    users,
    vehicle_types,
);
