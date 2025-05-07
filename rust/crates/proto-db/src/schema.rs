// @generated automatically by Diesel CLI.

diesel::table! {
    device (id) {
        id -> Integer,
        uid -> Text,
        description -> Text,
        address -> Text,
    }
}

diesel::table! {
    schedules (id) {
        id -> Integer,
        sid -> Integer,
        name -> Text,
        days -> Text,
        record_url -> Text,
        kind -> Integer,
        weeks -> Text,
        dates -> Text,
        times -> Text,
        month -> Nullable<Integer>,
        year -> Nullable<Integer>,
        volume -> Nullable<Double>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    device,
    schedules,
);
