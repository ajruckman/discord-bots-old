pub mod bots {
    table! {
        bots.attachment (id) {
            id -> Varchar,
            file_name -> Varchar,
            file_size_bytes -> Int4,
            url -> Varchar,
        }
    }

    table! {
        bots.message (id) {
            id -> Varchar,
            id_server -> Varchar,
            id_channel -> Varchar,
            id_author -> Varchar,
            time -> Timestamp,
            time_edited -> Nullable<Timestamp>,
            content -> Text,
            id_attachment -> Nullable<Array<Text>>,
        }
    }

    table! {
        bots.server (id) {
            id -> Varchar,
            name -> Text,
            url_icon -> Text,
        }
    }

    joinable!(message -> server (id_server));

    allow_tables_to_appear_in_same_query!(
        attachment,
        message,
        server,
    );
}
