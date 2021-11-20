use std::{env, fmt::Write};

use serenity::{
    async_trait,
    model::{
        channel::{Message, MessageType},
        gateway::Ready,
    },
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.kind == MessageType::InlineReply {
            println!(
                "{} 서버 {} 채널에서 {}#{}({}) 유저가 답장을 사용했습니다!\n> {}",
                msg.guild_id.unwrap_or_default(),
                msg.channel_id,
                msg.author.name,
                msg.author.discriminator,
                msg.author.id,
                msg.content,
            );
            // Deleting a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.delete(&ctx.http).await {
                println!("Error deleting message: {:?}", why);
            }
            let target_channel = env::var("DISCORD_CHANNEL").unwrap_or(String::from(""));
            let mut warning_message = String::from("저희 서버에서 답장 기능은 금지되어 있어요!");
            if target_channel != "" {
                write!(warning_message, " <#{}> 정독 부탁드려요!", target_channel).unwrap();
            }
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, warning_message).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    dotenv::dotenv().expect("Failed to load .env file");

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
