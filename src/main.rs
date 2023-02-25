use chrono::{Datelike, Local};
use poise::serenity_prelude as serenity;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const POSSIBLE_CHORES: [&str; 5] = [
    "           Trash",
    "       Recycling",
    "          Dishes",
    "Tidy First Floor",
    "  Pick Something",
];

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Displays the current weekly chore asasignments
#[poise::command(slash_command, prefix_command)]
async fn chores(ctx: Context<'_>) -> Result<(), Error> {
    let week = Local::now().iso_week().week() + 1;
    // let response = format!("Week: {}", week);
    let mut members = ctx
        .guild_id()
        .unwrap()
        .members(ctx, None, None)
        .await?
        .into_iter()
        .filter(|m| !m.user.bot)
        .map(|m| m.display_name().into_owned())
        .collect::<Vec<String>>();
    let num_members = members.len();
    members.rotate_left(week as usize % num_members);
    let members = members
        .into_iter()
        .enumerate()
        .map(|(i, name)| format!("`{}`: {}", POSSIBLE_CHORES[i], name))
        .collect::<Vec<_>>()
        .join("\n");
    ctx.say(format!("Chore order:\n{}", members)).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), chores()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MEMBERS,
        )
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
