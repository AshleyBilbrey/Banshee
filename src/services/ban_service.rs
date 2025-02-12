use std::collections::HashSet;

use poise::insert_owners_from_http;
use serenity::{
    all::{User, UserId},
    Error,
};

use crate::types::{self, Error};

use super::user_service::is_super_user;

async fn ban(
    ctx: &serenity::client::Context,
    user: User,
    reason: String,
) -> Result<bool, types::Error> {
    let owners: HashSet<UserId> = match ctx.http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else if let Some(owner) = &info.owner {
                owners.insert(owner.id);
            }
            owners
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    if is_super_user(&user.id).await?
        || owners.contains(&user.id)
        || ctx.http.get_current_user().await?.id == user.id
    {
        return Ok(false);
    }

    ban_user_db().await?;

    Ok(true)
}

async fn ban_user_db() -> Result<(), types::Error> {
    Ok(())
}
