mod cli;
mod handler;

use std::{collections::BinaryHeap, sync::Arc};

use tokio::{
    sync::Mutex,
    time::{Duration, interval},
};
use traq_ws_bot::{
    builder,
    openapi::{
        apis::{configuration::Configuration, message_api::post_message},
        models::PostMessageRequest,
    },
    utils::create_configuration,
};

use crate::handler::{
    handle_bot_message_stamps_updated, handle_direct_message_created, handle_message_created,
};

#[tokio::main]
async fn main() {
    let bat = std::env::var("BOT_ACCESS_TOKEN").unwrap();
    let conf = Arc::new(create_configuration(&bat));
    let gueue = Arc::new(Mutex::new(BinaryHeap::new()));

    {
        // 毎時 𝑩𝑰𝑮 𝑮𝑼𝑬𝑼𝑬 するスレ
        const GUEUE_CHANNEL: &str = "019b78f2-fa78-76a8-81f8-b93c2fcb4c86";
        let mut interval = interval(Duration::from_secs(3600));
        let conf = conf.clone();
        let gueue = gueue.clone();
        tokio::spawn(async move {
            loop {
                interval.tick().await;
                if let Some((_, user_name, message_id)) = gueue.lock().await.pop() {
                    let req = PostMessageRequest {
                        content: format!(
                            "@{} ぎゅー！\n\nhttps://q.trap.jp/messages/{}",
                            user_name, message_id
                        ),
                        embed: Some(true),
                    };
                    let _ = post_message(&conf, GUEUE_CHANNEL, Some(req)).await;
                }
            }
        });
    }

    let bot = builder(&bat)
        .insert_resource(Resource { conf, gueue })
        .on_direct_message_created_with_resource(handle_direct_message_created)
        .on_message_created_with_resource(handle_message_created)
        .on_bot_message_stamps_updated_with_resource(handle_bot_message_stamps_updated)
        .build();

    bot.start().await.unwrap();
}

struct Resource {
    conf: Arc<Configuration>,
    gueue: Arc<Mutex<BinaryHeap<(u8, String, String)>>>,
}
