use chorust::data::factures::RechercherFournisseurData;
use chorust::data::transverses::ConsulterCrDetailleData;
use chorust::enums::ChorusResponse;
use chorust::Chorus;
use getopts::Options;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = Options::new();
    opts.reqopt("f", "flow", "Flow to check", "FLOW");
    opts.reqopt("u", "username", "The Choruspro username", "USERNAME");
    opts.reqopt("p", "password", "The Choruspro password", "PASSWORD");
    opts.reqopt("c", "clientid", "The OAuth client id", "CLIENT_ID");
    opts.reqopt(
        "z",
        "clientsecret",
        "The OAuth client secret",
        "CLIENT_SECRET",
    );
    opts.optflag("", "sandbox", "Sandbox mode");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            println!("{}", opts.usage("arkhineo_get"));
            std::process::exit(1);
        }
    };

    let client_id = matches.opt_str("c").unwrap();
    let client_secret = matches.opt_str("z").unwrap();
    let username = matches.opt_str("u").unwrap();
    let password = matches.opt_str("p").unwrap();
    let flow = matches.opt_str("f").unwrap();

    let mut chorus = Chorus::default();
    chorus.switch_to_sandbox_mode(matches.opt_present("sandbox"));
    let response = chorus
        .connect_with_oauth(&client_id, &client_secret)
        .unwrap();

    if !response.http_code.is_successful() {
        println!("Error during authentication : {}", response.raw_data);
        return;
    }

    chorus.set_choruspro_account(&username, &password);

    let ccdd = ConsulterCrDetailleData {
        numero_flux_depot: Some(flow.clone()),
        syntaxe_flux: None,
    };

    let upload = chorus.consulter_cr_detaille(&ccdd).unwrap();

    match &upload.object {
        Some(ChorusResponse::Success(v)) => {
            println!("Flow uploaded with status {}", v.etat_courant_depot_flux)
        }
        Some(ChorusResponse::Error(v)) => println!("Error : {} - {}", v.code_retour, &v.libelle),
        Some(ChorusResponse::Unknown(v)) => println!("Unknown : {}", v),
        None => println!("No known response : {}", upload.raw_data),
    }

    let rfd = RechercherFournisseurData {
        numero_flux_depot: Some(flow.clone()),
        ..Default::default()
    };

    let upload = chorus.rechercher_fournisseur(&rfd).unwrap();

    match &upload.object {
        Some(ChorusResponse::Success(v)) => {
            if v.liste_factures.is_empty() {
                println!("No document found for this flow");
                return;
            }

            for facture in &v.liste_factures {
                println!(
                    "Document found : {} ({})",
                    facture.numero_facture.clone().unwrap_or_default(),
                    facture.statut.clone().unwrap_or_default()
                );
            }
        }
        Some(ChorusResponse::Error(v)) => println!("Error : {} - {}", v.code_retour, &v.libelle),
        Some(ChorusResponse::Unknown(v)) => println!("Unknown : {}", v),
        None => println!("No known response : {}", upload.raw_data),
    }
}
