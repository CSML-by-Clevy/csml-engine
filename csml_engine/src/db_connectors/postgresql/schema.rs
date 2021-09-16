table! {
    cmsl_bot_versions (id) {
        id -> Uuid,
        bot_id -> Varchar,
        bot -> Text,
        engine_version -> Varchar,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    csml_conversations (id) {
        id -> Uuid,
        bot_id -> Varchar,
        channel_id -> Varchar,
        user_id -> Varchar,
        flow_id -> Varchar,
        step_id -> Varchar,
        status -> Varchar,
        last_interaction_at -> Timestamp,
        updated_at -> Timestamp,
        created_at -> Timestamp,
        expires_at -> Nullable<Timestamp>,
    }
}

table! {
    csml_memories (id) {
        id -> Uuid,
        bot_id -> Varchar,
        channel_id -> Varchar,
        user_id -> Varchar,
        key -> Varchar,
        value -> Varchar,
        expires_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    csml_messages (id) {
        id -> Uuid,
        conversation_id -> Uuid,
        flow_id -> Varchar,
        step_id -> Varchar,
        direction -> Varchar,
        payload -> Varchar,
        content_type -> Varchar,
        message_order -> Int4,
        interaction_order -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
        expires_at -> Nullable<Timestamp>,
    }
}

table! {
    csml_states (id) {
        id -> Uuid,
        bot_id -> Varchar,
        channel_id -> Varchar,
        user_id -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        key -> Varchar,
        value -> Varchar,
        expires_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

joinable!(csml_messages -> csml_conversations (conversation_id));

allow_tables_to_appear_in_same_query!(
    cmsl_bot_versions,
    csml_conversations,
    csml_memories,
    csml_messages,
    csml_states,
);
