use std::env;

use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
    },
    prelude::*,
};

const HELP_MESSAGE: &str = "
Hello there, Human!

You have summoned me. Let's see about getting you what you need.

? Need technical help?
==> Post is the <#CHANNEL_ID> channel and other humans will assit you.

? Looking for the Code of Conduct?
==> Here it is: <Place_Holder>

? Something wrong?
==> You can flag an admin with @admin

I hope that resolves your issue!
-- Helpbot
";

const HELP_COMMAND: &str = "!help";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
// I don't understand why the following error ocurs when i add the #[tokio::main] to the main function. 
// Probably i have a solution for this but i don' know why it has compiled.
/* error: The default runtime flavor is `multi_thread`, but the `rt-multi-thread` feature is disabled.
  --> src\main.rs:49:1
   |
49 | #[tokio::main]
   | ^^^^^^^^^^^^^^
   |
   = note: this error originates in an attribute macro (in Nightly builds, run with -Z macro-backtrace for more info)

 */
#[tokio::main]
async fn main() {
    let token = env::var("DISCORD TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
