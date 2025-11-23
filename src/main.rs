mod cli;
mod handler;

use traq_ws_bot::{builder, utils::create_configuration};

use crate::handler::{handle_direct_message_created, handle_message_created};

#[tokio::main]
async fn main() {
    let bat = std::env::var("BOT_ACCESS_TOKEN").unwrap();
    let conf = create_configuration(&bat);

    let bot = builder(&bat)
        .insert_resource(conf)
        .on_direct_message_created_with_resource(handle_direct_message_created)
        .on_message_created_with_resource(handle_message_created)
        .build();

    bot.start().await.unwrap();
}
