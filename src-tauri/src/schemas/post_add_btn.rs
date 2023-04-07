use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostAddBtn {
    pub service_name: String,
    pub status: String,
    pub pending_printing: String,
    pub transaction_id: String,
    pub response_body: ResponseBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    pub total: String,
    pub result: Vec<Vec<String>>,
}
