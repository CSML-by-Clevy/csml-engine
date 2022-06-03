table! {
    cmsl_bot_versions (id) {
        id -> Binary,
        bot_id -> Text,
        bot -> Text,
        engine_version -> Text,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    csml_conversations (id) {
        id -> Binary,
        bot_id -> Text,
        channel_id -> Text,
        user_id -> Text,
        flow_id -> Text,
        step_id -> Text,
        status -> Text,
        last_interaction_at -> Timestamp,
        updated_at -> Timestamp,
        created_at -> Timestamp,
        expires_at -> Nullable<Timestamp>,
    }
}

table! {
    csml_memories (id) {
        id -> Binary,
        bot_id -> Text,
        channel_id -> Text,
        user_id -> Text,
        key -> Text,
        value -> Text,
        expires_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    csml_messages (id) {
        id -> Binary,
        conversation_id -> Binary,
        flow_id -> Text,
        step_id -> Text,
        direction -> Text,
        payload -> Text,
        content_type -> Text,
        message_order -> Integer,
        interaction_order -> Integer,
        updated_at -> Timestamp,
        created_at -> Timestamp,
        expires_at -> Nullable<Timestamp>,
    }
}

table! {
    csml_states (id) {
        id -> Binary,
        bot_id -> Text,
        channel_id -> Text,
        user_id -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        key -> Text,
        value -> Text,
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
