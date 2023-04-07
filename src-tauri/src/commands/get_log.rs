use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::{BufReader, Read, Write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    auth::login::{login, LoginRet},
    schemas::{get_log_schema::GetLogSchema, login_schema::AccessData},
    utils::string_utils::get_json,
};

use once_cell::sync::Lazy;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetLogResponse {
    message: String,
    created: bool,
}

static DOC_PATH: Lazy<String> = Lazy::new(|| env::temp_dir().display().to_string());
/// Retorna N linhas do conteudo do arquivo de log encontrado
///
/// ### Argumentos
/// * `url` - Uma String representando a url de acesso
/// *  `access_data` - Uma struct do tipo AccessData, relacionada as informações de login
/// ```
#[tauri::command]
pub async fn get_log(url: String, access_data: AccessData, total: i32) -> Result<String, String> {
    match action(url, access_data, total).await {
        Ok(data) => Ok(data),
        Err(e) => Err(e),
    }
}

async fn action(url: String, access_data: AccessData, total: i32) -> Result<String, String> {
    let file_resp = match get_file(url, access_data).await {
        Ok(a) => Ok(a),
        Err(e) => Err(e),
    }; // Pega o arquivo de retorno
    if file_resp.is_err() {
        Err(file_resp.unwrap_err())
    } else {
        let (file_path, zip_file) = file_resp.unwrap();
        let lines_per_file = if total <= 0 { 10000 } else { total }; // Limita a quantidade de linhas de retorno
        let reader = BufReader::new(zip_file);

        let mut archive = zip::ZipArchive::new(reader).expect("Erro ao ler o arquivo");

        // Deleta o arquivo ZIP
        fs::remove_file(file_path).expect("Error deleting file"); // Deleta o arquivo zip já carregado na memoria

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap(); // Captura o arquivo ainda dentro do .zip
            if file.is_file() && file.name().eq("server.log") {
                let mut data = vec![];
                file.read_to_end(&mut data).expect("Error reading file"); // Lê os bytes do arquivo e adiciona a variável "data"
                let original_content = String::from_utf8_lossy(&data).to_string(); // Converte os dados obtidos para uma string UTF-8
                let lines = original_content.lines().rev(); // Inverte as linhas para que sejam lidas de tras para frente

                let mut content = String::new();
                let mut count = 0;
                for line in lines {
                    if count > lines_per_file {
                        break;
                    }
                    content = content + "\n" + line;
                    count = count + 1;
                }

                let new_content = ((content.lines().rev()).collect::<Vec<&str>>()).join("\n"); // Reverte a string para o ordem natural que foi lida
                return Ok(new_content);
            }
        }

        Ok("".to_owned())
    }
}
/// Retorna uma tupla informando: caminho do arquivo+arquivo
///
/// ### Argumentos
///
/// * `url` - Uma String representando a url de acesso
/// *  `access_data` - Uma struct do tipo AccessData, relacionada as informações de login
/// ```
pub async fn get_file(url: String, access_data: AccessData) -> Result<(String, File), String> {
    let login_resp = match login(url.clone(), access_data.clone()).await {
        Ok(a) => Ok(a),
        Err(e) => Err(e),
    };
    if login_resp.is_err() {
        let error = login_resp.unwrap_err();
        Err(serde_json::to_string(&error).expect("Error Converting"))
    } else {
        let login_data = login_resp.unwrap();
        let LoginRet { root: _, client } = login_data.clone(); // Pega os dados de login

        let zip_file = get_zip_file_name(url.clone(), login_data)
            .await
            .expect("Erro ao retornar arquivo");

        let get_url = format!("{}/mge/downloadTempFile.mge?fileName={}", &url, zip_file); // URL de requisição + nome do arquivo gerado
        let response = client
            .get(get_url)
            .send()
            .await
            .expect("Erro sending request"); // Ffaz a requisição http para baixar o arquivo

        let json = get_json("./jsons/getLog.json"); // Pega o json modelo da pasta "jsons"

        let post_url = format!(
            "{}/mge/service.sbr?serviceName=MobileLoginSP.logout&outputType=json",
            &url
        ); // Formata a url para usar o token
        let _response_logout = client
            .post(post_url)
            .body(json)
            .send()
            .await
            .expect("Erro sending request"); // Faz a requisição http para logout(A VARIAVEL NÃO É USADA POR NÃO SER NECESSÁRIO)

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards"); // Pega a hora atual

        let file_name = format!("zip_{}.zip", current_time.as_millis()); // Nome do arquivo zip a ser criado
        let temp_path = DOC_PATH.to_owned();
        let file_path = format!("{}/{}", temp_path, file_name); // Caminho do arquivo zip a ser criado+nome

        let path = Path::new(&file_path); // Cria um novo "path" para referenciar o arquivo futuro

        let mut file = match File::create(&path) {
            Err(error) => panic!("Erro ao criar arquivo: {}", error),
            Ok(file) => file,
        }; // Tenta criar o arquivo permitindo SOMENTE a ESCRITA, caso falhe para a execução com um "panic"

        let data = response.bytes().await.expect("Erro parssing response"); // Pegas os dados da requisição http e passa para bytes(nesse ponto os dados retornados é o proprio arquivo ZIP)

        file.write_all(&data).expect("Error writing file"); // Escreve o zip para o novo zip apontado para a variavel "path"

        let ret_file = File::open(&file_path).expect("Couldn't open file"); // Tenta abrir o arquivo permitindo SOMENTE a LEITURA

        Ok((file_path, ret_file)) // Retorna a tupla de caminho do arquivo criado+arquivo criado instanciado
    }
}

/// Retorna o nome do arquivo .zip gerado pelo sankhya
///
/// ### Argumentos
/// * `url` - Uma String representando a url de acesso
/// *  `access_data` - Uma struct do tipo AccessData, relacionada as informações de login
/// ```
pub async fn get_zip_file_name(url: String, login_data: LoginRet) -> Result<String, String> {
    let LoginRet { root, client } = login_data; // pega os dados de login

    let jsession_token = String::from(root.response_body.jsessionid.field); // Pega o jsession ID
    let json = get_json("./jsons/getLog.json"); // Pega o json modelo da pasta "jsons"

    let last_char = url.chars().last().unwrap();
    let mut post_url = format!(
        "{}/mge/service.sbr?serviceName=AdministracaoServidorSP.getLog&outputType=json&mgeSession={}",
        &url, &jsession_token
    ); // Formata a url para usar o token
    if last_char.eq(&'/') {
        post_url = format!(
            "{}mge/service.sbr?serviceName=AdministracaoServidorSP.getLog&outputType=json&mgeSession={}",
            &url, &jsession_token
        ); // Formata a url para usar o token
    }

    let response = client
        .post(post_url.clone())
        .body(json)
        .send()
        .await
        .expect("Erro sending request"); // Faz a requisição http

    let resp = response
        .text()
        .await
        .expect("{\"message\":\"Erro during conversion\"}"); // tenta converter o arquivo para json

    let json_parsed: GetLogSchema = serde_json::from_str(&resp).unwrap(); // transforma o json em uma estrutura

    Ok(json_parsed.response_body.log.file_name) // retorna o nome do arquivo
}
