use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct ChorusError {
    #[serde(rename = "code_retour")]
    pub code_retour: i64,

    pub libelle: String,
}

impl Display for ChorusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.libelle)
    }
}
