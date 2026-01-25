//! Transfer model.

use chrono::NaiveDate;
use cm_core::economy::Money;
use cm_core::ids::{ClubId, PlayerId, TransferId};
use serde::{Deserialize, Serialize};

/// Transfer status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferStatus {
    Proposed,
    Negotiating,
    Accepted,
    Rejected,
    Completed,
    Cancelled,
}

/// Transfer record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub id: TransferId,
    pub player_id: PlayerId,
    pub from_club: ClubId,
    pub to_club: ClubId,
    pub fee: Money,
    pub date: NaiveDate,
    pub status: TransferStatus,
}

impl Transfer {
    pub fn new(
        player_id: PlayerId,
        from_club: ClubId,
        to_club: ClubId,
        fee: Money,
        date: NaiveDate,
    ) -> Self {
        let id = format!("TRF-{}-{}", date, player_id);
        Self {
            id: TransferId::new(id),
            player_id,
            from_club,
            to_club,
            fee,
            date,
            status: TransferStatus::Proposed,
        }
    }
}
