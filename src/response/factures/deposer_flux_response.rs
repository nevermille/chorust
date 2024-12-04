use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct DeposerFluxResponse {
    #[serde(rename = "codeRetour")]
    pub code_retour: i64,

    #[serde(rename = "dateDepot")]
    pub date_depot: String,

    #[serde(rename = "libelle")]
    pub libelle: String,

    #[serde(rename = "numeroFluxDepot")]
    pub numero_flux_depot: String,

    #[serde(rename = "syntaxeFlux")]
    pub syntaxe_flux: String,
}
