table! {
    /*dl_info (id) {
        id -> Integer,
        dl_url -> Text,
        dl_status -> Integer,
        dl_progress -> Nullable<Double>,
        dl_create_time -> Text,
        dl_end_time -> Text,
        dl_type -> Text,
        file_size -> Nullable<Text>,
        file_store_path -> Nullable<Text>,
        completed -> Bool,
        description -> Text,
    }*/
    dl_info {
        id -> Nullable<Integer>,
        description -> Nullable<Text>,
        completed -> Bool,
        dl_url -> Text,
        dl_status -> Text,
        dl_progress -> Nullable<Double>,
        dl_create_time -> Text,
        dl_end_time -> Text,
        dl_type -> Text,
        file_size -> Nullable<Text>,
        file_store_path -> Nullable<Text>,
    }
}



table! {
    sys_user (id) {
        id -> Integer,
        uname -> Text,
        pwd -> Text,
        email -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    dl_info,
    sys_user,
);
