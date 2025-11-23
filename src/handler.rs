use std::sync::Arc;

use clap::Parser;
use traq_ws_bot::{
    events::payload::{DirectMessageCreated, MessageCreated},
    openapi::{
        apis::{configuration::Configuration, message_api::post_message},
        models::PostMessageRequest,
    },
};

use crate::cli::Cli;

pub async fn handle_message_created(event: MessageCreated, configuration: Arc<Configuration>) {
    let content = match Cli::try_parse_from(event.message.plain_text.split_whitespace().skip(1)) {
        Ok(cli) => cli.run(),
        Err(e) => format!("```\n{}```", e.render()),
    };
    let _ = post_message(
        &configuration,
        &event.message.channel_id,
        Some(PostMessageRequest {
            content,
            embed: Some(false),
        }),
    )
    .await;
}

pub async fn handle_direct_message_created(
    event: DirectMessageCreated,
    configuration: Arc<Configuration>,
) {
    let content = match Cli::try_parse_from(event.message.plain_text.split_whitespace()) {
        Ok(cli) => cli.run(),
        Err(e) => format!("```\n{}```", e.render()),
    };
    let _ = post_message(
        &configuration,
        &event.message.channel_id,
        Some(PostMessageRequest {
            content,
            embed: Some(false),
        }),
    )
    .await;
}
