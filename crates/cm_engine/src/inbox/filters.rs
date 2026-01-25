//! Message filters (stub).

use super::message::{InboxMessage, MessageCategory};

/// Filter unread messages.
pub fn unread(messages: &[InboxMessage]) -> Vec<&InboxMessage> {
    messages.iter().filter(|m| !m.read).collect()
}

/// Filter by category.
pub fn by_category(messages: &[InboxMessage], category: MessageCategory) -> Vec<&InboxMessage> {
    messages
        .iter()
        .filter(|m| std::mem::discriminant(&m.category) == std::mem::discriminant(&category))
        .collect()
}
