use crate::response::definitions::{ParametresRetour, RetourRechercherFactureParFournisseur};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]

pub struct RechercherFournisseurResponse {
    #[serde(rename = "codeRetour")]
    pub code_retour: i32,

    pub libelle: String,

    #[serde(rename = "listeFactures")]
    #[serde(default)]
    pub liste_factures: Vec<RetourRechercherFactureParFournisseur>,

    #[serde(rename = "parametresRetour")]
    #[serde(default)]
    pub parametres_retour: Option<ParametresRetour>,
}
