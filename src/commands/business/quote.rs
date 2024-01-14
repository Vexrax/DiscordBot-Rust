use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Quote {
    pub quote: String,
    pub year: String,
    pub author: String,
    pub context: String,
}