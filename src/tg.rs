use crate::pure::*;
use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "This is the bot interacting with Pure app via telegram")]
    Help,
    #[command(description = "Start bot")]
    Start,
    #[command(description = "Stop bot")]
    Stop,
    #[command(description = "Set match count to stop bot")]
    SetMatchLimit(u8),
}
