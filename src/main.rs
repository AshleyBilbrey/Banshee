use poise::{serenity_prelude as serenity, Framework};

mod commands;
mod types;

/// List of available commands
fn get_commands() -> Vec<poise::Command<types::Data, types::Error>> {
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
    framework: &poise::Framework<types::Data, types::Error>,
) -> Result<types::Data, types::Error> {
    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
    let current_user: serenity::CurrentUserRef<'_> = ctx.cache.current_user();

    println!("Starting Banshee as {}", &current_user.tag());
    Ok(types::Data {})
}

#[tokio::main]
async fn main() {
    let token: String = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents: serenity::prelude::GatewayIntents = serenity::GatewayIntents::non_privileged();

    let framework: Framework<types::Data, types::Error> = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: get_commands(),
            ..Default::default()
        })
        .setup(
            |ctx: &serenity::Context,
             ready: &serenity::Ready,
             framework: &Framework<types::Data, types::Error>| {
                Box::pin(setup_framework(ctx, ready, framework))
            },
        )
        .build();

    let client: Result<serenity::Client, serenity::Error> =
        serenity::ClientBuilder::new(token, intents)
            .framework(framework)
            .await;
    client.unwrap().start().await.unwrap();
}
