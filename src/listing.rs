use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListingType {
    Server,
    Bot,
}

impl Display for ListingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListingType::Server => write!(f, "server"),
            ListingType::Bot => write!(f, "bot"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Tag {
    Server(ServerTag),
    Bot(BotTag),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ServerTag {
    Gaming,
    Anime,
    Music,
    Study,
    Coding,
    Art,
    Community,
    Roleplay,
    Memes,
    Technology,
    Social,
    Creative,
    Educational,
    Support,
}

impl ServerTag {
    pub fn as_str(&self) -> &str {
        match self {
            ServerTag::Gaming => "gaming",
            ServerTag::Anime => "anime",
            ServerTag::Music => "music",
            ServerTag::Study => "study",
            ServerTag::Coding => "coding",
            ServerTag::Art => "art",
            ServerTag::Community => "community",
            ServerTag::Roleplay => "roleplay",
            ServerTag::Memes => "memes",
            ServerTag::Technology => "technology",
            ServerTag::Social => "social",
            ServerTag::Creative => "creative",
            ServerTag::Educational => "educational",
            ServerTag::Support => "support",
        }
    }

    pub fn all() -> &'static [ServerTag] {
        &[
            ServerTag::Gaming,
            ServerTag::Anime,
            ServerTag::Music,
            ServerTag::Study,
            ServerTag::Coding,
            ServerTag::Art,
            ServerTag::Community,
            ServerTag::Roleplay,
            ServerTag::Memes,
            ServerTag::Technology,
            ServerTag::Social,
            ServerTag::Creative,
            ServerTag::Educational,
            ServerTag::Support,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BotTag {
    Utility,
    Moderation,
    Music,
    Economy,
    Fun,
    Leveling,
    Logging,
    Tickets,
    Automation,
    Analytics,
}

impl BotTag {
    pub fn as_str(&self) -> &str {
        match self {
            BotTag::Utility => "utility",
            BotTag::Moderation => "moderation",
            BotTag::Music => "music",
            BotTag::Economy => "economy",
            BotTag::Fun => "fun",
            BotTag::Leveling => "leveling",
            BotTag::Logging => "logging",
            BotTag::Tickets => "tickets",
            BotTag::Automation => "automation",
            BotTag::Analytics => "analytics",
        }
    }

    pub fn all() -> &'static [BotTag] {
        &[
            BotTag::Utility,
            BotTag::Moderation,
            BotTag::Music,
            BotTag::Economy,
            BotTag::Fun,
            BotTag::Leveling,
            BotTag::Logging,
            BotTag::Tickets,
            BotTag::Automation,
            BotTag::Analytics,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Listing {
    pub discord_server: DiscordServer,
    pub listing_type: ListingType,
    pub description: String,
    pub tags: Vec<Tag>,
    pub last_bumped_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateListingRequest {
    pub listing_type: ListingType,
    pub guild_id: String,
    pub description: String,
    pub tags: Tags,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateListingRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Tags>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {
    pub tags: Vec<Tag>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingsQuery {
    pub listing_type: Option<ListingType>,
    pub tags: Option<Tags>,
    pub search: Option<String>,
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingsResponse {
    pub listings: Vec<Listing>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BumpResponse {
    pub success: bool,
    pub message: String,
    pub next_bump_available_at: Option<String>,
    pub next_bump_in_seconds: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DiscordServer {
    pub id: String,
    pub name: String,
    pub invite_url: String,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetServersRequest {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetServersResponse {
    pub servers: Vec<DiscordServer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotBumpRequest {
    pub guild_id: String,
    pub user_id: String,
}
