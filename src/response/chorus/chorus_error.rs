use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct ChorusError {
    #[serde(rename = "codeRetour")]
    pub code_retour: i64,

    #[serde(rename = "libelle")]
    pub libelle: String,
}

impl Display for ChorusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.libelle)
    }
}
