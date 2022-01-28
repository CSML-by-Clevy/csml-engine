use crate::data::Client;

use log::{error, warn, info, debug, trace};
use std::io::Write;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum LogLvl {
    Error,
    Warn,
    Info,
    Debug,
    Trace
}

pub struct CsmlLog {
    bot_id: Option<String>,
    user_id: Option<String>,
    channel_id: Option<String>,
    flow: Option<String>,
    line: Option<u32>,
    message: String,
}

impl std::fmt::Debug for CsmlLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("");

        let mut debug_struct = ds.field("message", &self.message);

        if let Some(flow) = &self.flow {
            debug_struct = debug_struct.field("flow", flow);
        }
        if let Some(line) = &self.line {
            debug_struct = debug_struct.field("line", line);
        }

        if let Some(bot_id) = &self.bot_id {
            debug_struct = debug_struct.field("bot_id", bot_id);
        }
        if let Some(user_id) = &self.user_id {
            debug_struct = debug_struct.field("user_id", user_id);
        }
        if let Some(channel_id) = &self.channel_id {
            debug_struct = debug_struct.field("channel_id", channel_id);
        }

        debug_struct.finish()
    }
}

impl CsmlLog {
    pub fn new(
        client: Option<&Client>,
        flow: Option<String>,
        line: Option<u32>,
        message: String
    ) -> Self {
        let (bot_id, user_id, channel_id) = match client {
            Some(client) => (
                Some(client.bot_id.to_string()),
                Some(client.user_id.to_string()),
                Some(client.channel_id.to_string())
            ),
            None => (None, None, None)
        };

        let flow = match flow {
            Some(flow) => Some(flow),
            None => None
        };

        let line = match line {
            Some(line) => Some(line),
            None => None
        };

        Self {
            bot_id,
            user_id,
            channel_id,
            flow,
            line,
            message,
        }
    }
}

pub fn init_logger() {
    let env = env_logger::Env::default()
    .filter_or("CSML_LOG_LEVEL", "error");

    let _ = env_logger::Builder::from_env(env)
    .format(|buf, record| {
        let style = buf.default_level_style(record.level());

        let timestamp = buf.timestamp_millis();
        let path = record.target();

        writeln!(
            buf,
            "{} {} {} {}",
            style.value(record.level()),
            timestamp,
            path,
            record.args()
        )
    })
    .try_init();
}

pub fn csml_logger(
    log_message: CsmlLog,
    log_lvl: LogLvl
) {
    match log_lvl {
        LogLvl::Error => error!("{:#?}", log_message),
        LogLvl::Warn => warn!("{:#?}", log_message),
        LogLvl::Info => info!("{:#?}", log_message),
        LogLvl::Debug => debug!("{:#?}", log_message),
        LogLvl::Trace => trace!("{:#?}", log_message),
    }
}