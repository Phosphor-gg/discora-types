use serde::Deserialize;

// Discord API response for /users/@me/guilds/{guild_id}/member
#[derive(Debug, Deserialize)]
pub struct DiscordGuildMember {
    pub permissions: Option<String>, // Bitfield as string
}