use serde::{Serialize};
use reqwest::{Client};
use std::collections::{HashMap, HashSet};
use tokio::{spawn};
use futures::{stream, StreamExt}; // 0.3.8


use crate::utils::config::API_LOCATION;
use crate::utils::reference::Reference;

const INDEX_BATCH_SIZE: usize = 10;

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
    
    let responses = stream::iter(references_to_index).map(|reference| {
        let reference_vec = vec![reference.clone()];
        let client = client.clone();
        spawn(async move {
            let index_reference_request = IndexReferencesRequest {
                references: serde_json::to_string(&reference_vec).unwrap(),
            };
            let url = format!("{}/index", API_LOCATION);
            client.post(&url).json(&index_reference_request).send().await;
        })
    });

    responses.buffer_unordered(INDEX_BATCH_SIZE).collect::<Vec<_>>().await;
}
