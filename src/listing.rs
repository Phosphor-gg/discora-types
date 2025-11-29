use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListingType {
    Server,
    Bot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub id: String,
    pub listing_type: ListingType,
    pub owner_id: String,
    pub owner_username: String,
    pub discord_id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub invite_url: String,
    pub last_bumped_at: Option<String>,
    pub bump_count: i32,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateListingRequest {
    pub listing_type: ListingType,
    pub discord_id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub invite_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateListingRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub invite_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingsQuery {
    pub listing_type: Option<ListingType>,
    pub tags: Option<Vec<String>>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordServer {
    pub id: String,
    pub name: String,
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

// Predefined tags
pub const PREDEFINED_TAGS: &[&str] = &[
    "gaming",
    "anime",
    "music",
    "study",
    "coding",
    "art",
    "community",
    "roleplay",
    "memes",
    "technology",
    "utility-bot",
    "moderation-bot",
    "music-bot",
    "economy-bot",
    "fun-bot",
];
