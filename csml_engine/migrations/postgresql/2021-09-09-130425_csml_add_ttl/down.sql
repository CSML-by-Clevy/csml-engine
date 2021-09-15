CREATE TABLE csml_interactions (
  id uuid PRIMARY KEY,
  bot_id VARCHAR NOT NULL,
  channel_id VARCHAR NOT NULL,
  user_id VARCHAR NOT NULL,

  success BOOLEAN NOT NULL,
  event VARCHAR NOT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE csml_nodes (
  id uuid PRIMARY KEY,
  interaction_id uuid NOT NULL REFERENCES csml_interactions (id) ON DELETE CASCADE,
  conversation_id uuid NOT NULL REFERENCES csml_conversations (id) ON DELETE CASCADE,

  flow_id VARCHAR NOT NULL,
  step_id VARCHAR NOT NULL,
  next_flow VARCHAR DEFAULT NULL,
  next_step VARCHAR DEFAULT NULL,

  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE csml_conversations DROP COLUMN expires_at;
ALTER TABLE csml_messages DROP COLUMN expires_at;

