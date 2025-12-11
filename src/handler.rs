use std::sync::Arc;

use clap::Parser;
use traq_ws_bot::{
    events::payload::{BotMessageStampsUpdated, DirectMessageCreated, MessageCreated},
    openapi::{
        apis::{
            configuration::Configuration,
            message_api::{delete_message, edit_message, post_message},
        },
        models::PostMessageRequest,
    },
};

use crate::cli::Cli;

pub async fn handle_message_created(event: MessageCreated, configuration: Arc<Configuration>) {
    let content = match Cli::try_parse_from(event.message.plain_text.split_whitespace()) {
        Ok(cli) => cli.run().await,
        Err(e) => format!("```txt\n{}\n```", e.render().to_string().trim()),
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
    let content = match Cli::try_parse_from(
        format!("@BOT_hayatroid {}", event.message.plain_text).split_whitespace(),
    ) {
        Ok(cli) => cli.run().await,
        Err(e) => format!("```txt\n{}\n```", e.render().to_string().trim()),
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

pub async fn handle_bot_message_stamps_updated(
    event: BotMessageStampsUpdated,
    configuration: Arc<Configuration>,
) {
    const ME: &str = "d7ed5690-45fa-4f9e-956a-2165017445ac";
    const WASTEBASKET: &str = "bfb541b0-2dd0-4d35-b418-8433e4474e69";
    if event
        .stamps
        .iter()
        .any(|stamp| stamp.stamp_id == WASTEBASKET && stamp.user_id == ME && stamp.count > 0)
    {
        let _ = edit_message(
            &configuration,
            &event.message_id,
            Some(PostMessageRequest {
                content: "\u{200b}".to_string(),
                embed: Some(false),
            }),
        )
        .await;
        let _ = delete_message(&configuration, &event.message_id).await;
    }
}
