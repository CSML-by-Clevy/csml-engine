CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  client_id INTEGER NOT NULL REFERENCES client (id),
  interaction_id INTEGER NOT NULL REFERENCES interactions (id),
  conversation_id INTEGER NOT NULL REFERENCES conversations (id),

  flow_id VARCHAR NOT NULL,
  step_id VARCHAR NOT NULL,
  direction VARCHAR NOT NULL,
  payload VARCHAR NOT NULL,
  content_type VARCHAR NOT NULL,

  message_order INTEGER NOT NULL,
  interaction_order INTEGER NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
