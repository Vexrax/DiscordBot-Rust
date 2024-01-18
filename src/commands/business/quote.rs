use serde::{Deserialize, Serialize};

pub const QUOTE_DB_NAME: &str = "Quotes";
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Quote {
    pub quote: String,
    pub year: String,
    pub author: String,
    pub context: String,
}