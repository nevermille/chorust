use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct DeposerFluxResponse {
    #[serde(rename = "code_retour")]
    pub code_retour: i64,

    #[serde(rename = "dateDepot")]
    pub date_depot: String,

    #[serde(rename = "libelle")]
    pub libelle: String,

    #[serde(rename = "numeroFluxDepot")]
    #[serde(default)]
    pub numero_flux_depot: String,

    #[serde(rename = "syntaxeFlux")]
    #[serde(default)]
    pub syntaxe_flux: String,
}
