CREATE TABLE nodes (
  id SERIAL PRIMARY KEY,
  client_id INTEGER NOT NULL REFERENCES client (id),
  interaction_id INTEGER NOT NULL REFERENCES interactions (id),
  conversation_id INTEGER NOT NULL REFERENCES conversations (id),

  flow_id VARCHAR NOT NULL,
  step_id VARCHAR NOT NULL,
  next_flow VARCHAR DEFAULT NULL,
  next_step VARCHAR DEFAULT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
