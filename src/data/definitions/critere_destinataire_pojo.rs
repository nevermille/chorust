use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct CritereDestinatairePojo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "idDestinataire")]
    pub id_destinataire: Option<i64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "listeIdServiceExecutant")]
    pub liste_id_service_executant: Vec<i64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "raisonSocialeStuctDmd")]
    pub raison_sociale_stuct_dmd: Option<String>,
}
