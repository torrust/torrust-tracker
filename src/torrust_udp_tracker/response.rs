use crate::torrust_udp_tracker::TransactionId;

pub struct ErrorResponse {
    pub transaction_id: TransactionId,
    pub message: String,
}
