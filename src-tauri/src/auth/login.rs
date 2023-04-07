use reqwest::Client;
use serde::{Serialize, Deserialize};

use crate::api::post_login::post_login;
use crate::schemas::login_schema::{AccessData, Root};
#[derive(Clone, Debug)] // we add the Clone trait to Morpheus struct
pub struct LoginRet {
    pub root: Root,
    pub client: Client,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginError {
    pub message: String,
    pub ret_body: String,
}

pub async fn login(url: String, access_data: AccessData) -> Result<LoginRet, LoginError> {
    let get_login = post_login(url, access_data)
        .await
        .expect("Error getting data");
    let client = get_login.1;

    let data = get_login.0;

    let serde_struct: Result<Root, serde_json::Error> = serde_json::from_str(&data);
    if serde_struct.is_err() {
        let error = serde_struct.unwrap_err();

        let error_ret = LoginError {
            message: error.to_string(),
            ret_body: data,
        };
        Err(error_ret)
    } else {
        let ret = LoginRet {
            root: serde_struct.unwrap(),
            client: client,
        };

        Ok(ret)
    }
}
