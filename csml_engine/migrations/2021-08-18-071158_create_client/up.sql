CREATE TABLE client (
  id SERIAL PRIMARY KEY,
  bot_id VARCHAR NOT NULL REFERENCES cmsl_bot (id),
  channel_id VARCHAR NOT NULL,
  user_id VARCHAR NOT NULL
)
