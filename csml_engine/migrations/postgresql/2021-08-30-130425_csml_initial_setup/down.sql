DROP TRIGGER expire_conversations_table_trigger ON csml_conversations;
DROP FUNCTION expire_conversations_table();

-- DROP TRIGGER expire_messages_table_trigger ON csml_messages;
-- DROP FUNCTION expire_messages_table();

DROP TRIGGER expire_memories_table_trigger ON csml_memories;
DROP FUNCTION expire_memories_table();

DROP TRIGGER expire_states_table_trigger ON csml_states;
DROP FUNCTION expire_states_table();

DROP INDEX memory_client_key;

DROP TABLE csml_memories;
DROP TABLE csml_messages;
DROP TABLE csml_states;
DROP TABLE csml_conversations;
DROP TABLE cmsl_bot_versions;
