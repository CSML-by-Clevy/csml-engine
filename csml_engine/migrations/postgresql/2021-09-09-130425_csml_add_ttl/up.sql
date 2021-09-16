ALTER TABLE csml_conversations ADD COLUMN expires_at TIMESTAMP DEFAULT NULL;

ALTER TABLE csml_messages ADD COLUMN expires_at TIMESTAMP DEFAULT NULL;
ALTER TABLE csml_messages DROP COLUMN IF EXISTS interaction_id;

DROP TABLE csml_nodes;
DROP TABLE csml_interactions;