use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use nvimsence_rs::handler::EventHandler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DiscordIpcClient::new("877708715850104892")?;
    let mut event_handler = EventHandler::new(client)?;

    event_handler.rich_presence.connect().unwrap_or(());
    event_handler.listen()?;

    Ok(())
}
