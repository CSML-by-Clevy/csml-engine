
CREATE TABLE cmsl_bot_versions (
  id uuid PRIMARY KEY,
  bot_id VARCHAR NOT NULL,

  bot TEXT NOT NULL,
  engine_version VARCHAR NOT NULL,

  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE csml_conversations (
  id uuid PRIMARY KEY,
  bot_id VARCHAR NOT NULL,
  channel_id VARCHAR NOT NULL,
  user_id VARCHAR NOT NULL,

  flow_id VARCHAR NOT NULL,
  step_id VARCHAR NOT NULL,
  status VARCHAR NOT NULL,

  last_interaction_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

  expires_at TIMESTAMP DEFAULT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE FUNCTION expire_conversations_table() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
  DELETE FROM csml_conversations WHERE expires_at < NOW();
  RETURN NEW;
END;
$$;

CREATE TRIGGER expire_conversations_table_trigger
    AFTER INSERT ON csml_conversations
    EXECUTE PROCEDURE expire_conversations_table();

CREATE TABLE csml_messages (
  id uuid PRIMARY KEY,
  conversation_id uuid NOT NULL REFERENCES csml_conversations (id) ON DELETE CASCADE,

  flow_id VARCHAR NOT NULL,
  step_id VARCHAR NOT NULL,
  direction VARCHAR NOT NULL,
  payload VARCHAR NOT NULL,
  content_type VARCHAR NOT NULL,

  message_order INTEGER NOT NULL,
  interaction_order INTEGER NOT NULL,

  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
  -- expires_at TIMESTAMP,


-- CREATE FUNCTION expire_messages_table() RETURNS trigger
--     LANGUAGE plpgsql
--     AS $$
-- BEGIN
--   DELETE FROM csml_messages WHERE expires_at < NOW();
--   RETURN NEW;
-- END;
-- $$;

-- CREATE TRIGGER expire_messages_table_trigger
--     AFTER INSERT ON csml_messages
--     EXECUTE PROCEDURE expire_messages_table();

CREATE TABLE csml_memories (
  id uuid PRIMARY KEY,
  bot_id VARCHAR NOT NULL,
  channel_id VARCHAR NOT NULL,
  user_id VARCHAR NOT NULL,

  key VARCHAR NOT NULL,
  value VARCHAR NOT NULL,

  expires_at TIMESTAMP DEFAULT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE FUNCTION expire_memories_table() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
  DELETE FROM csml_memories WHERE expires_at < NOW();
  RETURN NEW;
END;
$$;

CREATE TRIGGER expire_memories_table_trigger
    AFTER INSERT ON csml_memories
    EXECUTE PROCEDURE expire_memories_table();

CREATE UNIQUE INDEX memory_client_key ON csml_memories (bot_id, channel_id, user_id, key);

CREATE TABLE csml_states (
  id uuid PRIMARY KEY,
  bot_id VARCHAR NOT NULL,
  channel_id VARCHAR NOT NULL,
  user_id VARCHAR NOT NULL,

  type VARCHAR NOT NULL,
  key VARCHAR NOT NULL,
  value VARCHAR NOT NULL,

  expires_at TIMESTAMP DEFAULT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE FUNCTION expire_states_table() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
  DELETE FROM csml_states WHERE expires_at < NOW();
  RETURN NEW;
END;
$$;

CREATE TRIGGER expire_states_table_trigger
    AFTER INSERT ON csml_states
    EXECUTE PROCEDURE expire_states_table();