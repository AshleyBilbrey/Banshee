use poise::{samples::register_in_guild, serenity_prelude as serenity, Framework};
use services::{config_service::get_report_server, event_handler_service::event_handler};
use std::env;

mod commands;
mod entities;
mod services;
mod types;

/// List of available commands
fn get_public_commands() -> Vec<types::Command> {
    vec![
        commands::allow::allow(),
        commands::allowlist::allowlist(),
        commands::pet::pet(),
        commands::help::help(),
        commands::removelegacybans::removelegacybans(),
        commands::report::report(),
        commands::supers::supers(),
        commands::unallow::unallow(),
        commands::user::user(),
    ]
}

fn get_private_commands() -> Vec<types::Command> {
    vec![
        commands::register::register(),
        commands::supercmd::supercmd(),
        commands::unsuper::unsuper(),
        commands::ban::ban(),
        commands::unban::unban(),
    ]
}

/// Initialize the poise framework
async fn setup_framework(
    ctx: &serenity::Context,
    _ready: &serenity::Ready,
    _framework: &types::Framework,
) -> Result<types::Data, types::Error> {
    poise::builtins::register_globally(ctx, &get_public_commands()).await?;

    register_in_guild(ctx, &get_private_commands(), get_report_server().await).await?;

    let current_user = ctx.cache.current_user();
    println!("Starting Banshee as {}", &current_user.tag());
    Ok(types::Data {})
}

/// Create the framework
fn create_framework() -> types::Framework {
    let mut all_commands = get_public_commands();
    all_commands.extend(get_private_commands());
    Framework::builder()
        .options(poise::FrameworkOptions {
            commands: all_commands,
            event_handler: |ctx, event, framework_ctx, _u| {
                Box::pin(event_handler(ctx, event, framework_ctx))
            },
            initialize_owners: true,
            ..Default::default()
        })
        .setup(|ctx, ready, framework| Box::pin(setup_framework(ctx, ready, framework)))
        .build()
}

/// Create the Discord client
async fn create_client(
    token: String,
    intents: serenity::GatewayIntents,
    framework: types::Framework,
) -> serenity::Client {
    serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Failed to create client")
}

#[tokio::main]
async fn main() {
    // Load environment variables and setup intents
    let token = env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::GUILD_MEMBERS;

    // Initialize framework and client
    let framework = create_framework();
    let mut client = create_client(token, intents, framework).await;

    // Start the bot
    client.start().await.expect("Failed to start bot");
}
