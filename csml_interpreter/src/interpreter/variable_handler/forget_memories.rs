use crate::data::{
    ast::*, data::Data, MessageData
};

pub fn forget_scope_memories(forget_mem: &ForgetMemory, data: &mut Data) {
    match forget_mem {
        ForgetMemory::ALL => {
            data.step_vars.clear();
            data.context.current.clear();
        },
        ForgetMemory::SINGLE(memory) => {
            data.step_vars.remove(&memory.ident);
            data.context.current.remove(&memory.ident);
        },
        ForgetMemory::LIST(memories) => {
            for memory in memories.iter() {
                data.step_vars.remove(&memory.ident);
                data.context.current.remove(&memory.ident);
            }
        },
    }
}

pub fn remove_message_data_memories(forget_mem: &ForgetMemory, message_data: &mut MessageData) {
    match forget_mem {
        ForgetMemory::ALL => {
            message_data.memories = None;
        },
        ForgetMemory::SINGLE(memory) => {
            if let Some(memories_to_remember) = &message_data.memories {
                let memories =
                    memories_to_remember.iter()
                            .fold(vec![], |mut acc, mem| {
                                    if mem.key != memory.ident {
                                        acc.push(mem.to_owned());
                                    }
                                    acc
                                }
                            );

                message_data.memories = Some(memories);
            }
        },
        ForgetMemory::LIST(memories) => {
            let list_of_memories: Vec<String> = memories.iter().map(|mem| mem.ident.to_owned()).collect();

            if let Some(memories_to_remember) = &message_data.memories {
                let memories =
                    memories_to_remember.iter()
                            .fold(vec![], |mut acc, mem| {
                                    if !list_of_memories.contains(&mem.key) {
                                        acc.push(mem.to_owned());
                                    }
                                    acc
                                }
                            );

                message_data.memories = Some(memories);
            }
        },
    }
}