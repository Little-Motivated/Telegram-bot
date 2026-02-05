use std::sync::Arc;
use tokio::sync::Mutex;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let text = match msg.text() {
            Some(t) => t.to_string(),
            None => {
                bot.send_message(msg.chat.id, "Waiting...").await?;
                return Ok(());
            }
        };

        bot.send_message(msg.chat.id, text).await?;
        Ok(())
    })
    .await;
}
