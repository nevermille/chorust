use anyhow::bail;
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct DeposerFluxData {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "avecSignature")]
    pub avec_signature: Option<bool>,

    #[serde(default)]
    #[serde(rename = "fichierFlux")]
    pub fichier_flux: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "idUtilisateurCourant")]
    pub id_utilisateur_courant: Option<i64>,

    #[serde(default)]
    #[serde(rename = "nomFichier")]
    pub nom_fichier: String,

    #[serde(default)]
    #[serde(rename = "syntaxeFlux")]
    pub syntaxe_flux: String,
}

impl DeposerFluxData {
    pub fn add_file(&mut self, file_path: &str) -> anyhow::Result<()> {
        let file = Path::new(file_path);

        // We verify if the file exists
        if !file.exists() {
            bail!("File {file_path} does not exist");
        }

        // Filename extraction
        self.nom_fichier = file
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // File contents
        let file_contents = std::fs::read(file)?;

        // Encoding to base64
        self.fichier_flux = BASE64_STANDARD.encode(file_contents);

        Ok(())
    }

    pub fn force_extension(&mut self, extension: &str) {
        if !self.nom_fichier.ends_with(extension) {
            self.nom_fichier.push_str(extension);
        }
    }
}
