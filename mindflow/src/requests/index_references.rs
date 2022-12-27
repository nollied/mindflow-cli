use serde::{Serialize};
use reqwest::{Client};
use std::collections::HashMap;

use crate::utils::config::API_LOCATION;
use crate::utils::reference::Reference;

#[derive(Serialize)]
pub(crate) struct IndexReferencesRequest {
    pub(crate) references: String,
}

pub(crate) async fn request_index_references(client: &Client, resolved_references: &HashMap<String, Reference>, unindexed_hashes: &Vec<String>) {
    let mut references_to_index = Vec::new();
    for hash in unindexed_hashes {
        references_to_index.push(resolved_references.get(hash).unwrap());
    }
    let url = format!("{}/index", API_LOCATION);
    let index_reference_request: IndexReferencesRequest = IndexReferencesRequest {
        references: serde_json::to_string(&references_to_index).unwrap(),
    };
    let res = client.post(&url).json(&index_reference_request).send().await;
    match res {
        Ok(_) => {
            println!("Indexed references");
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}