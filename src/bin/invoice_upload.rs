use chorust::data::factures::DeposerFluxData;
use chorust::enums::ChorusResponse;
use chorust::Chorus;
use getopts::Options;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = Options::new();
    opts.reqopt("f", "file", "Invoice to send", "FILE");
    opts.reqopt("u", "username", "The Choruspro username", "USERNAME");
    opts.reqopt("p", "password", "The Choruspro password", "PASSWORD");
    opts.reqopt("c", "clientid", "The OAuth client id", "CLIENT_ID");
    opts.reqopt(
        "z",
        "clientsecret",
        "The OAuth client secret",
        "CLIENT_SECRET",
    );

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
    let file = matches.opt_str("f").unwrap();

    let mut chorus = Chorus::default();
    chorus.switch_to_sandbox_mode(true);
    let response = chorus
        .connect_with_oauth(&client_id, &client_secret)
        .unwrap();

    if !response.http_code.is_successful() {
        println!("Error during authentication : {}", response.raw_data);
        return;
    }

    chorus.set_choruspro_account(&username, &password);

    let mut dfd = DeposerFluxData::default();
    dfd.add_file(&file).unwrap();
    dfd.syntaxe_flux = "IN_DP_E2_UBL_INVOICE_MIN".to_string();
    dfd.force_extension(".xml");

    let upload = chorus.deposer_flux(&dfd).unwrap();

    match &upload.object {
        Some(ChorusResponse::Success(v)) => println!(
            "{} flow uploaded with id {}",
            &v.syntaxe_flux, &v.numero_flux_depot
        ),
        Some(ChorusResponse::Error(v)) => println!("Error : {} - {}", v.code_retour, &v.libelle),
        Some(ChorusResponse::Unknown(v)) => println!("Unknown : {}", v),
        None => println!("No known response : {}", upload.raw_data),
    }
}
