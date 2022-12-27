use serde::{Deserialize, Serialize};
use reqwest::{Client};
use std::collections::HashMap;

use crate::utils::config::API_LOCATION;
use crate::utils::reference::Reference;

#[derive(Serialize)]
pub(crate) struct TrimmedReference {
    pub(crate) hash: String,
    pub(crate) reference_type: String,
}

#[derive(Serialize)]
pub(crate) struct UnindexedReferenceRequest {
    pub(crate) references: String,
}

#[derive(Deserialize)]
pub(crate) struct UnindexedReferencesResponse {
    pub(crate) unindexed_hashes: Vec<String>,
}

pub(crate) async fn request_unindexed_references(client: &Client, resolved_references: &HashMap<String, Reference>) -> Result<UnindexedReferencesResponse, reqwest::Error> {
    let mut unindexed_references_payload = Vec::new();
    for (k, v) in resolved_references {
        unindexed_references_payload.push(TrimmedReference {
            hash: k.clone(),
            reference_type: v.get_type().clone(),
        });
    }    
    let unindexed_references_payload: UnindexedReferenceRequest = UnindexedReferenceRequest {
        references: serde_json::to_string(&unindexed_references_payload).unwrap(),
    };
    let url = format!("{}/unindexed", API_LOCATION);
    let res = client.post(&url)
        .json(&unindexed_references_payload)
        .send()
        .await?
        .json::<UnindexedReferencesResponse>()
        .await?;

    Ok(res)
}