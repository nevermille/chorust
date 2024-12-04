use chorust::enums::OAuthResponse;
use chorust::Chorus;
use getopts::Options;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = Options::new();
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

    let mut chorus = Chorus::default();
    chorus.switch_to_sandbox_mode(true);
    let response = chorus
        .connect_with_oauth(&client_id, &client_secret)
        .unwrap();

    match response.object {
        None => println!("No response returned"),
        Some(OAuthResponse::Success(v)) => println!("Success, bearer : {}", v.access_token),
        Some(OAuthResponse::Error(v)) => println!("Error : {}", v.error),
        Some(OAuthResponse::Unknown(v)) => println!("Unknown response : {}", v),
    }
}
