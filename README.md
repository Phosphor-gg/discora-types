# Discora Types

Shared type definitions for the Discora platform.

## Purpose

This crate provides common types used across all Discora services:
- `discora-backend` (API server)
- `discora-discord-bot` (Discord bot)
- `discora-website` (WASM frontend)

## Features

- Type-safe data structures with Serde serialization
- Consistent types across frontend and backend
- Request/response DTOs for API endpoints
- Shared enums and constants

## Modules

### `auth.rs`
- `User` - User account information
- `Claims` - JWT token claims
- `AuthResponse` - OAuth2 response
- `DiscordUser` - Discord API user data
- `UserInfo` - Public user information

### `listing.rs`
- `Listing` - Server/bot listing
- `ListingType` - Enum: Server or Bot
- `CreateListingRequest` - Create listing payload
- `UpdateListingRequest` - Update listing payload
- `SearchListingsResponse` - Search results
- `BumpResponse` - Bump operation result
- `PREDEFINED_TAGS` - Available tags constant

### `report.rs`
- `Report` - Report of inappropriate listing
- `ReportStatus` - Enum: Pending, Resolved, Dismissed
- `CreateReportRequest` - Create report payload
- `ReportActionRequest` - Admin action payload

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
discora-types = { path = "../discora-types" }
```

Example:

```rust
use discora_types::listing::{Listing, ListingType, PREDEFINED_TAGS};
use discora_types::auth::User;

// Use shared types
let listing_type = ListingType::Server;
let tags = PREDEFINED_TAGS;
```

## Rust Edition

Uses Rust 2024 Edition for consistency with other Discora crates.

## Dependencies

- `serde` - Serialization/deserialization
- `chrono` - Date/time handling
- `uuid` - Unique identifiers
