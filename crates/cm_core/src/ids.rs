//! Type-safe IDs for all entities.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Macro to create newtype ID wrappers.
macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(pub String);

        impl $name {
            pub fn new(s: impl Into<String>) -> Self {
                Self(s.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&String> for $name {
            fn from(s: &String) -> Self {
                Self(s.clone())
            }
        }
    };
}

define_id!(PlayerId);
define_id!(ClubId);
define_id!(StaffId);
define_id!(NationId);
define_id!(CompetitionId);
define_id!(StadiumId);
define_id!(RefereeId);
define_id!(MatchId);
define_id!(ContractId);
define_id!(TransferId);
