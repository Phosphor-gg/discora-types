use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReportStatus {
    Pending,
    Reviewed,
    Dismissed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: String,
    pub listing_id: String,
    pub listing_name: String,
    pub reporter_user_id: String,
    pub reporter_username: String,
    pub reason: String,
    pub status: ReportStatus,
    pub admin_notes: Option<String>,
    pub reviewed_by_user_id: Option<String>,
    pub reviewed_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReportRequest {
    pub listing_id: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateReportRequest {
    pub status: ReportStatus,
    pub admin_notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportsResponse {
    pub reports: Vec<Report>,
    pub total: u64,
}
