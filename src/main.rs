//! ![MODBOT logo][logo]
//!
//! ![Rust version][rust-version]
//! ![Rust edition][rust-edition]
//! ![License][license-badge]
//!
//! MODBOT is a Discord bot for [mod.io] using [`modio-rs`] and [`serenity`].
//!
//!
//! [rust-version]: https://img.shields.io/badge/rust-1.31%2B-blue.svg
//! [rust-edition]: https://img.shields.io/badge/edition-2018-red.svg
//! [license-badge]: https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg
//! [logo]: https://raw.githubusercontent.com/nickelc/modio-bot/master/logo.png
//! [mod.io]: https://mod.io
//! [`modio-rs`]: https://github.com/nickelc/modio-rs
//! [`serenity`]: https://github.com/serenity-rs/serenity
#![deny(rust_2018_idioms)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use dotenv::dotenv;
use serenity::framework::standard::{help_commands, DispatchError, StandardFramework};

#[macro_use]
mod macros;

mod commands;
mod db;
mod error;
#[rustfmt::skip]
mod schema;
mod util;

use commands::subs;
use commands::{Game, ListGames, ListMods, ModInfo, Popular};
use util::*;

const DATABASE_URL: &str = "DATABASE_URL";
const DISCORD_BOT_TOKEN: &str = "DISCORD_BOT_TOKEN";
const MODIO_HOST: &str = "MODIO_HOST";
const MODIO_API_KEY: &str = "MODIO_API_KEY";
const MODIO_TOKEN: &str = "MODIO_TOKEN";

const DEFAULT_MODIO_HOST: &str = "https://api.mod.io/v1";

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn try_main() -> CliResult {
    dotenv().ok();
    env_logger::init();

    let (mut client, modio, mut rt) = util::initialize()?;

    let games_cmd = ListGames::new(modio.clone(), rt.executor());
    let game_cmd = Game::new(modio.clone(), rt.executor());
    let mods_cmd = ListMods::new(modio.clone(), rt.executor());
    let mod_cmd = ModInfo::new(modio.clone(), rt.executor());
    let popular_cmd = Popular::new(modio.clone(), rt.executor());
    let list_subs_cmd = subs::List::new(modio.clone(), rt.executor());
    let subscribe_cmd = subs::Subscribe::new(modio.clone(), rt.executor());
    let unsubscribe_cmd = subs::Unsubscribe::new(modio.clone(), rt.executor());

    rt.spawn(subs::task(&client, modio.clone(), rt.executor()));

    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.prefix("~")
                    .dynamic_prefix(util::dynamic_prefix)
                    .on_mention(true)
            })
            .simple_bucket("simple", 1)
            .group("General", |g| {
                g.cmd("about", commands::basic::About)
                    .cmd("prefix", commands::basic::Prefix)
                    .cmd("invite", commands::basic::Invite)
                    .cmd("guide", commands::basic::Guide)
            })
            .group("mod.io", |g| {
                g.cmd("games", games_cmd)
                    .cmd("game", game_cmd)
                    .cmd("mods", mods_cmd)
                    .cmd("mod", mod_cmd)
                    .cmd("popular", popular_cmd)
                    .cmd("subscriptions", list_subs_cmd)
                    .cmd("subscribe", subscribe_cmd)
                    .cmd("unsubscribe", unsubscribe_cmd)
            })
            .on_dispatch_error(|_, msg, error| match error {
                DispatchError::NotEnoughArguments { .. } => {
                    let _ = msg.channel_id.say("Not enough arguments.");
                }
                DispatchError::LackingRole | DispatchError::LackOfPermissions(_) => {
                    let _ = msg
                        .channel_id
                        .say("You have insufficient rights for this command.");
                }
                DispatchError::RateLimited(_) => {
                    let _ = msg.channel_id.say("Try again in 1 second.");
                }
                _ => {}
            })
            .help(help_commands::with_embeds),
    );
    client.start()?;
    Ok(())
}
