use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct ConsulterCrDetailleResponse {
    #[serde(default)]
    #[serde(rename = "codeInterfaceDepotFlux")]
    pub code_interface_depot_flux: String,

    #[serde(rename = "codeRetour")]
    pub code_retour: i32,

    #[serde(default)]
    #[serde(rename = "dateDepotFlux")]
    pub date_depot_flux: String,

    #[serde(default)]
    #[serde(rename = "dateHeureEtatCourantFlux")]
    pub date_heure_etat_courant_flux: String,

    #[serde(default)]
    #[serde(rename = "etatCourantDepotFlux")]
    pub etat_courant_depot_flux: String,

    pub libelle: String,

    #[serde(default)]
    #[serde(rename = "listeErreurDP")]
    pub liste_erreur_dp: Vec<ConsulterCRDetailleResponseErreurDP>,

    #[serde(default)]
    #[serde(rename = "listeErreurTechnique")]
    pub liste_erreur_technique: Vec<ConsulterCRDetailleResponseErreurTechnique>,

    #[serde(default)]
    #[serde(rename = "nomFichier")]
    pub nom_fichier: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ConsulterCRDetailleResponseErreurDP {
    #[serde(default)]
    #[serde(rename = "identifiantDestinataire")]
    pub identifiant_destinataire: String,

    #[serde(default)]
    #[serde(rename = "identifiantFournisseur")]
    pub identifiant_fournisseur: String,

    #[serde(default)]
    #[serde(rename = "libelleErreurDP")]
    pub libelle_erreur_dp: String,

    #[serde(default)]
    #[serde(rename = "numeroDP")]
    pub numero_dp: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ConsulterCRDetailleResponseErreurTechnique {
    #[serde(default)]
    #[serde(rename = "codeErreur")]
    pub code_erreur: String,

    #[serde(default)]
    #[serde(rename = "libelleErreur")]
    pub libelle_erreur: String,

    #[serde(default)]
    #[serde(rename = "natureErreur")]
    pub nature_erreur: String,
}
