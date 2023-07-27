use reqwest::Client;
use std::error::Error;

use crate::{
    schemas::login_schema::AccessData,
    utils::{replace_param::replace_param, string_utils::get_json},
};

pub async fn post_login(
    url: String,
    access_data: AccessData,
) -> Result<(String, Client), Box<dyn Error>> {
    // let resp = reqwest::get("https://httpbin.org/ip").await?.text().await?;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("{\"message\":\"Erro ao iniciar o client\"}");
    let AccessData { username, password } = access_data;

    let mut json = get_json("./jsons/login.json");

    json = replace_param(&json, "username", username);
    json = replace_param(&json, "password", password);

    let post_url = format!(
        "{}/mge/service.sbr?serviceName=MobileLoginSP.login&outputType=json",
        url
    );

    let response = client.post(post_url).body(json).send().await?;

    let body = response.text().await?;

    Ok((body, client))
}
