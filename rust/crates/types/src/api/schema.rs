// @generated automatically by Diesel CLI.

diesel::table! {
    avs (id) {
        id -> Int4,
        #[max_length = 255]
        unique_id -> Varchar,
        status -> Int4,
        lat -> Nullable<Float8>,
        lng -> Nullable<Float8>,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        kind -> Int4,
        pending -> Int4,
        slots -> Text,
        networks -> Nullable<Text>,
        mem_total -> Nullable<Text>,
        mem_free -> Nullable<Text>,
        disk_total -> Nullable<Text>,
        disk_free -> Nullable<Text>,
        cpu_temp -> Nullable<Text>,
    }
}

diesel::table! {
    avs_port (id) {
        id -> Int4,
        avs_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        speaker -> Int4,
    }
}

diesel::table! {
    blacklist_token (id) {
        id -> Int4,
        token -> Text,
    }
}

diesel::table! {
    city (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        province_id -> Int4,
    }
}

diesel::table! {
    docs_credentials (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::table! {
    forgot_password (id) {
        id -> Int4,
        #[max_length = 255]
        uuid -> Varchar,
        user_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    mobile_session (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Text,
    }
}

diesel::table! {
    mobile_session_pending (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Text,
    }
}

diesel::table! {
    package (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        max_devices -> Int4,
    }
}

diesel::table! {
    province (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    records (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamp,
        description -> Nullable<Text>,
        file_url -> Text,
        #[max_length = 255]
        hash -> Varchar,
        user_id -> Int4,
        status -> Int4,
        duration -> Text,
        sender -> Nullable<Int4>,
    }
}

diesel::table! {
    role (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    schedules (id) {
        id -> Int4,
        name -> Text,
        days -> Array<Nullable<Int4>>,
        records_id -> Int4,
        device_ids -> Array<Nullable<Int4>>,
        kind -> Int4,
        weeks -> Array<Nullable<Int4>>,
        dates -> Array<Nullable<Int4>>,
        times -> Array<Nullable<Text>>,
        user_id -> Int4,
        month -> Nullable<Int4>,
        year -> Nullable<Int4>,
        volumes -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    subscription (id) {
        id -> Int4,
        user_id -> Int4,
        package_id -> Int4,
        order_date -> Timestamp,
        expire_date -> Timestamp,
    }
}

diesel::table! {
    user_group (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        parent_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        image_url -> Nullable<Varchar>,
        role_id -> Int4,
        device_ids -> Array<Nullable<Int4>>,
        city_id -> Nullable<Int4>,
        user_group_ids -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    verify (id) {
        id -> Int4,
        uuid -> Text,
        mobile -> Int4,
        session_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    web_session (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Text,
    }
}

diesel::table! {
    web_session_pending (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Text,
    }
}

diesel::joinable!(avs_port -> avs (avs_id));
diesel::joinable!(city -> province (province_id));
diesel::joinable!(records -> users (user_id));
diesel::joinable!(schedules -> records (records_id));
diesel::joinable!(schedules -> users (user_id));
diesel::joinable!(subscription -> package (package_id));
diesel::joinable!(subscription -> users (user_id));
diesel::joinable!(users -> city (city_id));
diesel::joinable!(users -> role (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    avs,
    avs_port,
    blacklist_token,
    city,
    docs_credentials,
    forgot_password,
    mobile_session,
    mobile_session_pending,
    package,
    province,
    records,
    role,
    schedules,
    subscription,
    user_group,
    users,
    verify,
    web_session,
    web_session_pending,
);
