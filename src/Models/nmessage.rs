pub struct NMessage {
    pub message_type: MessageTypeEnum,
    pub subscription_id: Option<String>,
    pub event: Option<NEvent>,
    pub filter: Option<Filter>,
    pub notice_message: Option<String>,
    pub is_event_duplicate: Option<bool>,
    pub success: Option<bool>,
    pub nevent_id: Option<String>,
    pub stats: Option<Stats>,
    pub context: MessageContextEnum,
}