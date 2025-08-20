pub struct Data {} // User data
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Framework = poise::Framework<Data, Error>;
pub type Command = poise::Command<Data, Error>;
pub enum ReportStatus {
    Open = 0,
    Banned = 1,
    Dismissed = 2,
}
