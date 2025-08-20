use std::env;

use ::serenity::all::ChannelId;
use ::serenity::all::GuildId;

pub async fn get_report_server() -> GuildId {
    let server_id_str = env::var("ADMIN_SERVER_ID").expect("missing ADMIN_SERVER_ID");
    GuildId::new(server_id_str.parse::<u64>().unwrap())
}

pub async fn get_report_channel() -> ChannelId {
    let channel_id_str = env::var("REPORT_CHANNEL_ID").expect("missing REPORT_CHANNEL_ID");
    ChannelId::new(channel_id_str.parse::<u64>().unwrap())
}
