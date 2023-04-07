use reqwest::{
    multipart::{self, Form},
    Body,
};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{
    auth::login::{login, LoginRet},
    schemas::{
        login_schema::AccessData, post_add_btn::PostAddBtn,
        post_cria_modulo_schema::PostCriaModuloSchema,
    },
    utils::{replace_param::replace_param, string_utils::get_json},
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetLogResponse {
    message: String,
    created: bool,
}

/// Retorna N linhas do conteudo do arquivo de log encontrado
///
/// ### Argumentos
/// * `url` - Uma String representando a url de acesso
/// *  `access_data` - Uma struct do tipo AccessData, relacionada as informações de login
/// ```
#[tauri::command]
pub async fn post_modulo_java(
    url: String,
    access_data: AccessData,
) -> Result<(String, String), String> {
    match action(url, access_data).await {
        Ok(data) => Ok(data),
        Err(e) => Err(e),
    }
}

async fn action(url: String, access_data: AccessData) -> Result<(String, String), String> {
    let login_resp = match login(url.clone(), access_data.clone()).await {
        Ok(a) => Ok(a),
        Err(e) => Err(e),
    };
    if login_resp.is_err() {
        let error = login_resp.unwrap_err();
        Err(serde_json::to_string(&error).expect("Erro ao incluir modulo"))
    } else {
        let login_data = login_resp.ok().unwrap();
        let original_content = post_modulo(url.clone(), &login_data).await; // Converte os dados obtidos para uma string UTF-8

        if original_content.is_err() {
            Err(serde_json::to_string(&"Erro").expect("Error Converting"))
        } else {
            let result = original_content.ok().unwrap();
            let LoginRet { root, client } = login_data.clone();
            let jsession_token = String::from(root.response_body.jsessionid.field); // Pega o jsession ID
            let last_char = url.chars().last().unwrap();
            post_file(
                (&url).to_owned(),
                (&jsession_token).to_owned(),
                last_char,
                (&client).to_owned(),
            )
            .await;
            post_add_jar(
                (&url).to_owned(),
                (&jsession_token).to_owned(),
                last_char,
                (&client).to_owned(),
                &result,
            )
            .await;

            let btn_result = post_add_btn(
                (&url).to_owned(),
                (&jsession_token).to_owned(),
                last_char,
                (&client).to_owned(),
                &result,
            )
            .await;
            if btn_result.is_ok() {
                let btn_id = btn_result.unwrap();
                return Ok((result, btn_id));
            };

            Err(serde_json::to_string(&"Erro").expect("Error Converting"))
        }
    }
}

/// Retorna codigo do modulo dentro do sankhya
///
/// ### Argumentos
/// * `url` - Uma String representando a url de acesso
/// *  `access_data` - Uma struct do tipo AccessData, relacionada as informações de login
/// ```
pub async fn post_modulo(url: String, login_data: &LoginRet) -> Result<String, String> {
    let LoginRet { root, client } = login_data; // pega os dados de login

    let jsession_token = String::from(&root.response_body.jsessionid.field); // Pega o jsession ID
    let json = get_json("./jsons/postCriaModulo.json"); // Pega o json modelo da pasta "jsons"

    let last_char = url.chars().last().unwrap();
    let endpoint = "mge/service.sbr?serviceName=DatasetSP.save&outputType=json&mgeSession=";
    let mut post_url = format!("{}/{}{}", &url, &endpoint, &jsession_token); // Formata a url para usar o token
    if last_char.eq(&'/') {
        post_url = format!("{}{}{}", &url, &endpoint, &jsession_token); // Formata a url para usar o token
    }
    let response = client
        .post(post_url)
        .body(json)
        .send()
        .await
        .expect("Erro sending request"); // Faz a requisição http

    let resp = response
        .text_with_charset("utf-8")
        .await
        .expect("{\"message\":\"Erro during conversion\"}"); // tenta converter o arquivo para json

    let json_parsed: PostCriaModuloSchema = serde_json::from_str(&resp).unwrap(); // transforma o json em uma estrutura

    if json_parsed.status_message.is_some() {
        if let Some(value) =
            get_existing_module(url.clone(), jsession_token, last_char, &client).await
        {
            return value;
        }
    }
    let response = (&json_parsed).response_body.as_ref().unwrap();
    let result = response.result.get(0).unwrap().get(0).unwrap().to_owned();
    Ok(result) // retorna o nome do arquivo
}

async fn get_existing_module(
    url: String,
    jsession_token: String,
    last_char: char,
    client: &reqwest::Client,
) -> Option<Result<String, String>> {
    let json_get_modulo_java = get_json("./jsons/getModuloJava.json");

    let endpoint_modulo =
        "mge/service.sbr?serviceName=DatasetSP.loadRecords&outputType=json&mgeSession=";

    let mut get_url = format!("{}/{}{}", &url, &endpoint_modulo, &jsession_token);

    if last_char.eq(&'/') {
        // Formata a url para usar o token
        get_url = format!("{}{}{}", &url, &endpoint_modulo, &jsession_token);
    }

    let module = client
        .post(get_url)
        .body(json_get_modulo_java)
        .send()
        .await
        .expect("Erro sending request");

    let resp_module = module
        .text()
        .await
        .expect("{\"message\":\"Erro during conversion\"}");

    let json_module_parsed: PostCriaModuloSchema = serde_json::from_str(&resp_module).unwrap();

    let response = (&json_module_parsed).response_body.as_ref().unwrap();
    let result = response.result.get(0);
    if result.is_some() {
        let ret = result.unwrap().get(0).unwrap().to_owned();
        return Some(Ok(ret));
    }
    return None;
}

async fn post_file(url: String, jsession_token: String, last_char: char, client: reqwest::Client) {
    let endpoint_modulo =
        "mge/sessionUpload.mge?sessionkey=ModuloJava&fitem=S&salvar=S&useCache=N&mgeSession=";

    let mut get_url = format!("{}/{}{}", &url, &endpoint_modulo, &jsession_token);

    if last_char.eq(&'/') {
        // Formata a url para usar o token
        get_url = format!("{}{}{}", &url, &endpoint_modulo, &jsession_token);
    }
    let multi = multipart().await.unwrap();
    client
        .post(get_url)
        .multipart(multi)
        .send()
        .await
        .expect("Erro sending request");
}

async fn post_add_jar(
    url: String,
    jsession_token: String,
    last_char: char,
    client: reqwest::Client,
    cod_module: &String,
) {
    let mut json = get_json("./jsons/postModulo.json");
    let file_name = "UtilityLibJava.jar";
    json = replace_param(&json, "codModulo", cod_module.to_owned());
    json = replace_param(&json, "moduleId", "ModuloJava".to_owned());
    json = replace_param(&json, "fileName", file_name.to_owned());

    let endpoint_modulo = "mge/service.sbr?serviceName=DatasetSP.save&outputType=json&mgeSession=";

    let mut get_url = format!("{}/{}{}", &url, &endpoint_modulo, &jsession_token);

    if last_char.eq(&'/') {
        // Formata a url para usar o token
        get_url = format!("{}{}{}", &url, &endpoint_modulo, &jsession_token);
    }

    client
        .post(get_url)
        .body(json)
        .send()
        .await
        .expect("Erro sending request");
}

async fn multipart() -> Result<Form, String> {
    let file = File::open("./jsons/UtilityLibJava.jar")
        .await
        .expect("Erro ao abrir arquivo");

    // read file body stream
    let stream = FramedRead::new(file, BytesCodec::new());
    let file_body = Body::wrap_stream(stream);

    //make form part of file
    let some_file = multipart::Part::stream(file_body)
        .file_name("UtilityLibJava.jar")
        .mime_str("text/plain")
        .expect("Erro ao montar arquivo");

    //create the multipart form
    let form = multipart::Form::new().part("arquivo", some_file);

    Ok(form)
}

async fn post_add_btn(
    url: String,
    jsession_token: String,
    last_char: char,
    client: reqwest::Client,
    cod_module: &String,
) -> Result<String, String> {
    let mut json = get_json("./jsons/postBtnJava.json");
    json = replace_param(&json, "codModulo", cod_module.to_owned());

    if let Some(value) = get_existing_btn(
        url.clone(),
        (&jsession_token).to_owned(),
        last_char,
        &client,
    )
    .await
    {
        return value;
    }
    let endpoint = "mge/service.sbr?serviceName=DatasetSP.save&outputType=json&mgeSession=";

    let mut post_url = format!("{}/{}{}", &url, &endpoint, &jsession_token);

    if last_char.eq(&'/') {
        // Formata a url para usar o token
        post_url = format!("{}{}{}", &url, &endpoint, &jsession_token);
    }

    let response = client
        .post(post_url)
        .body(json)
        .send()
        .await
        .expect("Erro sending request");
    let resp_btn = response.text().await.expect("Error converting");
    let json_btn_parsed: PostAddBtn = serde_json::from_str(&resp_btn).unwrap();

    let response = json_btn_parsed.response_body.to_owned();
    let result = response.result.get(0).unwrap().get(0).unwrap().to_owned();
    Ok(result)
}

async fn get_existing_btn(
    url: String,
    jsession_token: String,
    last_char: char,
    client: &reqwest::Client,
) -> Option<Result<String, String>> {
    let json_get_modulo_java = get_json("./jsons/getBtn.json");

    let endpoint_modulo =
        "mge/service.sbr?serviceName=DatasetSP.loadRecords&outputType=json&mgeSession=";

    let mut get_url = format!("{}/{}{}", &url, &endpoint_modulo, &jsession_token);

    if last_char.eq(&'/') {
        // Formata a url para usar o token
        get_url = format!("{}{}{}", &url, &endpoint_modulo, &jsession_token);
    }

    let module = client
        .post(get_url)
        .body(json_get_modulo_java)
        .send()
        .await
        .expect("Erro sending request");

    let resp_module = module
        .text()
        .await
        .expect("{\"message\":\"Erro during conversion\"}");

    let json_module_parsed: PostCriaModuloSchema = serde_json::from_str(&resp_module).unwrap();
    println!("{}", &resp_module);
    let response = (&json_module_parsed).response_body.as_ref().unwrap();
    let result = response.result.get(0);
    if result.is_some() {
        let ret = result.unwrap().get(0).unwrap().to_owned();
        return Some(Ok(ret));
    }
    return None;
}
