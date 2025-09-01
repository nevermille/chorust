use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ParametresRetour {
    #[serde(rename = "nbResultatsParPage")]
    #[serde(default)]
    pub nb_resultats_par_page: Option<i32>,

    #[serde(rename = "pageCourante")]
    #[serde(default)]
    pub page_courante: Option<i32>,

    #[serde(default)]
    pub pages: Option<i32>,

    #[serde(default)]
    pub total: Option<i32>,
}
