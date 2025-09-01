use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ConsulterCrDetailleData {
    /// The flow number
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "numeroFluxDepot")]
    pub numero_flux_depot: Option<String>,

    /// The flow syntax
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "syntaxeFlux")]
    pub syntaxe_flux: Option<String>,
}
