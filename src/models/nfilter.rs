use chrono::{DateTime, Local};
pub struct NFilter {
    pub event_ids: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
    pub kinds: Option<Vec<i32>>,
    pub tagged_event_ids: Option<Vec<String>>,
    pub tagged_public_keys: Option<Vec<String>>,
    pub tagged_keywords: Option<Vec<String>>,
    pub since: Option<DateTime<Local>>,
    pub until: Option<DateTime<Local>>,
    pub limit: Option<i32>,
    pub search: Option<String>,
}