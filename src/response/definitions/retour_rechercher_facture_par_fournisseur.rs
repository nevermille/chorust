use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct RetourRechercherFactureParFournisseur {
    #[serde(rename = "affactureurCode")]
    #[serde(default)]
    pub affactureur_code: Option<String>,

    #[serde(rename = "affactureurRaisonSociale")]
    #[serde(default)]
    pub affactureur_raison_sociale: Option<String>,

    #[serde(rename = "affactureurTypeIdentifiant")]
    #[serde(default)]
    pub affactureur_type_identifiant: Option<String>,

    #[serde(rename = "codeDestinataire")]
    #[serde(default)]
    pub code_destinataire: Option<String>,

    #[serde(rename = "codeFournisseur")]
    #[serde(default)]
    pub code_fournisseur: Option<String>,

    #[serde(rename = "codeServiceExecutant")]
    #[serde(default)]
    pub code_service_executant: Option<String>,

    #[serde(rename = "codeServiceFournisseur")]
    #[serde(default)]
    pub code_service_fournisseur: Option<String>,

    #[serde(rename = "codeValideur1")]
    #[serde(default)]
    pub code_valideur_1: Option<String>,

    #[serde(rename = "codeValideur2")]
    #[serde(default)]
    pub code_valideur_2: Option<String>,

    #[serde(rename = "commentaireEtatCourant")]
    #[serde(default)]
    pub commentaire_etat_courant: Option<String>,

    #[serde(rename = "coordBancairesFournisseurCleIban")]
    #[serde(default)]
    pub coord_bancaires_fournisseur_cle_iban: Option<String>,

    #[serde(rename = "coordBancairesFournisseurCleRib")]
    #[serde(default)]
    pub coord_bancaires_fournisseur_cle_rib: Option<String>,

    #[serde(rename = "coordBancairesFournisseurCodeBanque")]
    #[serde(default)]
    pub coord_bancaires_fournisseur_code_banque: Option<String>,

    #[serde(rename = "coordBancairesFournisseurCodePays")]
    #[serde(default)]
    pub coord_bancaires_fournisseur_code_pays: Option<String>,

    #[serde(rename = "coordBancairesFournisseurCompteBancaire")]
    #[serde(default)]
    pub coord_bancaires_fournisseur_compte_bancaire: Option<String>,

    #[serde(rename = "coordBancairesFournisseurNomCb")]
    #[serde(default)]
    pub coord_bancaires_fournisseur_nom_cb: Option<String>,

    #[serde(rename = "dateDepot")]
    #[serde(default)]
    pub date_depot: Option<String>,

    #[serde(rename = "dateFacture")]
    #[serde(default)]
    pub date_facture: Option<String>,

    #[serde(rename = "dateHeureEtatCourant")]
    #[serde(default)]
    pub date_heure_etat_courant: Option<String>,

    #[serde(rename = "dateValidation1")]
    #[serde(default)]
    pub date_validation_1: Option<String>,

    #[serde(rename = "dateValidation2")]
    #[serde(default)]
    pub date_validation_2: Option<String>,

    #[serde(rename = "designationDestinataire")]
    #[serde(default)]
    pub designation_destinataire: Option<String>,

    #[serde(rename = "designationFournisseur")]
    #[serde(default)]
    pub designation_fournisseur: Option<String>,

    #[serde(default)]
    pub devise: Option<String>,

    #[serde(rename = "idDestinataire")]
    #[serde(default)]
    pub id_destinataire: Option<i64>,

    #[serde(rename = "idServiceExecutant")]
    #[serde(default)]
    pub id_service_executant: Option<i64>,

    #[serde(rename = "identifiantFactureCPP")]
    #[serde(default)]
    pub identifiant_facture_cpp: Option<i64>,

    #[serde(rename = "modeDepot")]
    #[serde(default)]
    pub mode_depot: Option<String>,

    #[serde(rename = "montantAPayer")]
    #[serde(default)]
    pub montant_a_payer: Option<f64>,

    #[serde(rename = "montantHT")]
    #[serde(default)]
    pub montant_ht: Option<f64>,

    #[serde(rename = "montantTTC")]
    #[serde(default)]
    pub montant_ttc: Option<f64>,

    #[serde(rename = "nomPrenomUtilisateurCreateur")]
    #[serde(default)]
    pub nom_prenom_utilisateur_createur: Option<String>,

    #[serde(rename = "nomServiceExecutant")]
    #[serde(default)]
    pub nom_service_executant: Option<String>,

    #[serde(rename = "nomServiceFournisseur")]
    #[serde(default)]
    pub nom_service_fournisseur: Option<String>,

    #[serde(rename = "nomValideur1")]
    #[serde(default)]
    pub nom_valideur_1: Option<String>,

    #[serde(rename = "nomValideur2")]
    #[serde(default)]
    pub nom_valideur_2: Option<String>,

    #[serde(rename = "numeroBonCommande")]
    #[serde(default)]
    pub numero_bon_commande: Option<String>,

    #[serde(rename = "numeroFacture")]
    #[serde(default)]
    pub numero_facture: Option<String>,

    #[serde(rename = "numeroFactureOrigine")]
    #[serde(default)]
    pub numero_facture_origine: Option<String>,

    #[serde(rename = "numeroFluxDepot")]
    #[serde(default)]
    pub numero_flux_depot: Option<String>,

    #[serde(rename = "numeroMarche")]
    #[serde(default)]
    pub numero_marche: Option<String>,

    #[serde(rename = "prenomValideur1")]
    #[serde(default)]
    pub prenom_valideur_1: Option<String>,

    #[serde(rename = "prenomValideur2")]
    #[serde(default)]
    pub prenom_valideur_2: Option<String>,

    #[serde(rename = "raisonSocialeValideur1")]
    #[serde(default)]
    pub raison_sociale_valideur_1: Option<String>,

    #[serde(rename = "raisonSocialeValideur2")]
    #[serde(default)]
    pub raison_sociale_valideur_2: Option<String>,

    #[serde(rename = "rejetTraite")]
    #[serde(default)]
    pub rejet_traite: Option<bool>,

    #[serde(default)]
    pub statut: Option<String>,

    #[serde(default)]
    pub taille: Option<i64>,

    #[serde(rename = "typeDemandePaiement")]
    #[serde(default)]
    pub type_demande_paiement: Option<String>,

    #[serde(rename = "typeFacture")]
    #[serde(default)]
    pub type_facture: Option<String>,

    #[serde(rename = "typeIdentifiantFournisseur")]
    #[serde(default)]
    pub type_identifiant_fournisseur: Option<String>,

    #[serde(rename = "typeIdentifiantValideur1")]
    #[serde(default)]
    pub type_identifiant_valideur_1: Option<String>,

    #[serde(rename = "typeIdentifiantValideur2")]
    #[serde(default)]
    pub type_identifiant_valideur_2: Option<String>,
}
