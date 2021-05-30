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

fn main() {
    println!("Hello, world!");
}
