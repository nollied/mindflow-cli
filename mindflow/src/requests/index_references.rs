use serde::{Serialize};
use reqwest::{Client};
use std::collections::{HashMap, HashSet};

use crate::utils::config::API_LOCATION;
use crate::utils::reference::Reference;

#[derive(Serialize)]
pub(crate) struct IndexReferencesRequest {
    pub(crate) references: String,
}

pub(crate) async fn request_index_references(client: &Client, resolved_references: &HashMap<String, Reference>, unindexed_hashes: &Vec<String>) {
    // Create a vector of size resolved_references.keys() and fill it with None
    let references_to_index: Vec<Reference> = unindexed_hashes
        .iter()
        .filter_map(|k| resolved_references.get(k))
        .cloned()
        .collect();
        
    let index_reference_request: IndexReferencesRequest = IndexReferencesRequest {
        references: serde_json::to_string(&references_to_index).unwrap(),
    };
    let url = format!("{}/index", API_LOCATION);
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