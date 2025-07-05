use serde::{Deserialize, Serialize};
use serde_valid::Validate;
#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ProjectService {}
