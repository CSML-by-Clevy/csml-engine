table! {
    client (id) {
        id -> Int4,
        bot_id -> Varchar,
        channel_id -> Varchar,
        user_id -> Varchar,
    }
}

table! {
    cmsl_bot (id) {
        id -> Varchar,
        version_id -> Varchar,
        bot -> Text,
        engine_version -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    conversations (id) {
        id -> Int4,
        client_id -> Int4,
        flow_id -> Varchar,
        step_id -> Varchar,
        status -> Varchar,
        last_interaction_at -> Timestamp,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    interactions (id) {
        id -> Int4,
        client_id -> Int4,
        success -> Bool,
        event -> Varchar,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    memories (id) {
        id -> Int4,
        client_id -> Int4,
        key -> Varchar,
        value -> Varchar,
        expires_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

table! {
    messages (id) {
        id -> Int4,
        client_id -> Int4,
        interaction_id -> Int4,
        conversation_id -> Int4,
        flow_id -> Varchar,
        step_id -> Varchar,
        direction -> Varchar,
        payload -> Varchar,
        content_type -> Varchar,
        message_order -> Int4,
        interaction_order -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    nodes (id) {
        id -> Int4,
        client_id -> Int4,
        interaction_id -> Int4,
        conversation_id -> Int4,
        flow_id -> Varchar,
        step_id -> Varchar,
        next_flow -> Nullable<Varchar>,
        next_step -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

table! {
    states (id) {
        id -> Int4,
        client_id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        key -> Varchar,
        value -> Varchar,
        expires_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

joinable!(client -> cmsl_bot (bot_id));
joinable!(conversations -> client (client_id));
joinable!(interactions -> client (client_id));
joinable!(memories -> client (client_id));
joinable!(messages -> client (client_id));
joinable!(messages -> conversations (conversation_id));
joinable!(messages -> interactions (interaction_id));
joinable!(nodes -> client (client_id));
joinable!(nodes -> conversations (conversation_id));
joinable!(nodes -> interactions (interaction_id));
joinable!(states -> client (client_id));

allow_tables_to_appear_in_same_query!(
    client,
    cmsl_bot,
    conversations,
    interactions,
    memories,
    messages,
    nodes,
    states,
);
