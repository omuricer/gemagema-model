use psn_api_rs::{models::PSNUser, psn::PSN, traits::PSNRequest, types::PSNInner};

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    let refresh_token = String::from("your refresh token");

    // https://www.playstation.com/ja-jp/
    // https://ca.account.sony.com/api/v1/ssocookie
    let npsso = String::from("KtDqFe2RwvmXS4oZzlcgOat9U6hvfnwfW4OTpJJ779bgPoIFEopo0VBpoPhSWtd7");

    let client = PSN::new_client().expect("Failed to build http client");

    // construct a PSNInner object,add credentials and call auth to generate tokens.
    let mut psn_inner = PSNInner::new();
    psn_inner
        .set_region("jp".to_owned()) // <- set to a psn region server suit your case. you can leave it as default which is hk
        .set_lang("jp".to_owned()) // <- set to a language you want the response to be. default is en
        .set_self_online_id(String::from("nqi37696")) // <- this is used to generate new message thread. safe to leave unset if you don't need to send any PSN message.
        // .set_self_online_id(String::from("guts.inaq@gmail.com")) // <- this is used to generate new message thread. safe to leave unset if you don't need to send any PSN message.
        .add_refresh_token(refresh_token) // <- If refresh_token is provided then it's safe to ignore add_npsso and call .auth() directly.
        .add_npsso(npsso); // <- npsso is used only when refresh_token is not working or not provided.

    psn_inner = psn_inner
        .auth(client)
        .await
        .unwrap_or_else(|e| panic!("{:?}", e));

    println!(
        "Authentication Success! These are your info from PSN network: \r\n{:#?} ",
        psn_inner
    );

    // let user = psn_inner
    //     .get_profile::<PSNUser>(&client, "Hakoom")
    //     .await
    //     .unwrap_or_else(|e| panic!("{:?}", e));

    // println!("Example finished. Got user info : \r\n{:#?}", user);

    Ok(())
    // psn struct is dropped at this point so it's better to store your access_token and refresh_token here to make them reusable.
}
