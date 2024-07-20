pub struct NEvent {
    pub id: String,
    pub pubkey: Option<String>,
    pub created_at: i64,
    pub kind: Option<KindEnum>,
    pub tags: Vec<EventTag>,
    pub content: Option<String>,
    pub sig: Option<String>,
}