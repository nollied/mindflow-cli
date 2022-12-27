use serde::{Deserialize, Serialize};
use reqwest::{Client};
use std::collections::HashMap;

use crate::utils::config::API_LOCATION;
use crate::utils::reference::Reference;

#[derive(Serialize)]
pub(crate) struct QueryRequest {
    pub(crate) query_text: String,
    pub(crate) reference_hashes: Vec<String>,
}

#[derive(Deserialize)]
pub(crate) struct QueryResponse {
    pub(crate) text: String,
}

pub(crate) async fn request_query(client:&Client, query_text: String, resolved_references: &HashMap<String, Reference>) -> Result<QueryResponse, reqwest::Error> {
    let query = QueryRequest {
        query_text,
        reference_hashes: resolved_references.keys().map(|k| k.to_string()).collect::<Vec<String>>(),
    };
    let res = client
        .post(&format!("{}/query", API_LOCATION))
        .json(&query)
        .send()
        .await?
        .json::<QueryResponse>()
        .await?;
    
    Ok(res)
}