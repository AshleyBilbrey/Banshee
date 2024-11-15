use poise::{serenity_prelude as serenity, Framework};
use std::env;

mod commands;
mod entities;
mod services;
mod types;

/// List of available commands
fn get_commands() -> Vec<types::Command> {
    vec![
        commands::age::age(),
        commands::pet::pet(),
        commands::help::help(),
        commands::report::report(),
    ]
}

/// Initialize the poise framework
async fn setup_framework(
    ctx: &serenity::Context,
    _ready: &serenity::Ready,
    framework: &types::Framework,
) -> Result<types::Data, types::Error> {
    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
    let current_user = ctx.cache.current_user();
    println!("Starting Banshee as {}", &current_user.tag());
    Ok(types::Data {})
}

/// Create the framework
fn create_framework() -> types::Framework {
    Framework::builder()
        .options(poise::FrameworkOptions {
            commands: get_commands(),
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
    let intents = serenity::GatewayIntents::non_privileged();

    // Initialize framework and client
    let framework = create_framework();
    let mut client = create_client(token, intents, framework).await;

    // Start the bot
    client.start().await.expect("Failed to start bot");
}
