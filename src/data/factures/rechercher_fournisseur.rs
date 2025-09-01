use crate::data::definitions::{
    CritereDestinatairePojo, ParametresRechercherFactureParFournisseur,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct RechercherFournisseurData {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cadreFacturation")]
    pub cadre_facturation: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "coordonneeBancaire")]
    pub coordonnee_bancaire: Option<i64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "idFournisseur")]
    pub id_fournisseur: Option<i64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "idServiceFournisseur")]
    pub id_service_fournisseur: Option<i64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "idStructureValideur")]
    pub id_structure_valideur: Option<i64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "idUtilisateurCourant")]
    pub id_utilisateur_courant: Option<i64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "idUtilisateurCreateur")]
    pub id_utilisateur_createur: Vec<i64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "listeDestinataire")]
    pub liste_destinataire: Vec<CritereDestinatairePojo>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "modeDepot")]
    pub mode_depot: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "montantApayerMax")]
    pub montant_a_payer_max: Option<f64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "montantApayerMin")]
    pub montant_a_payer_min: Option<f64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "montantHTMax")]
    pub montant_ht_max: Option<f64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "montantHTMin")]
    pub montant_ht_min: Option<f64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "montantTTCMax")]
    pub montant_ttc_max: Option<f64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "montantTTCMin")]
    pub montant_ttc_min: Option<f64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "numeroBonCommande")]
    pub numero_bon_commande: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "numeroFacture")]
    pub numero_facture: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "numeroFactureOrigine")]
    pub numero_facture_origine: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "numeroFluxDepot")]
    pub numero_flux_depot: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "numeroMarche")]
    pub numero_marche: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "periodeDateDepotAu")]
    pub periode_date_depot_au: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "periodeDateDepotDu")]
    pub periode_date_depot_du: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "periodeDateFactureAu")]
    pub periode_date_facture_au: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "periodeDateFactureDu")]
    pub periode_date_facture_du: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "periodeDateHeureEtatCourantAu")]
    pub periode_date_heure_etat_courant_au: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "periodeDateHeureEtatCourantDu")]
    pub periode_date_heure_etat_courant_du: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rechercheFactureParFournisseur")]
    pub recherche_facture_par_fournisseur: Option<ParametresRechercherFactureParFournisseur>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rechercheSirenDestinataire")]
    pub recherche_siren_destinataire: Option<bool>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "recupererTaille")]
    pub recuperer_taille: Option<bool>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rejetTraite")]
    pub rejet_traite: Option<bool>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "statutCourant")]
    pub statut_courant: Vec<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "typeFacture")]
    pub type_facture: Option<String>,
}
