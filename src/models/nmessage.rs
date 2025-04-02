use crate::models::nevent::NEvent;
use crate::models::nfilter::NFilter;
use crate::enums::messagetypeenum::MessageTypeEnum;

pub struct NMessage {
    pub message_type: MessageTypeEnum,
    pub subscription_id: Option<String>,
    pub event: Option<NEvent>,
    pub filter: Option<NFilter>,
    pub message: Option<String>
}