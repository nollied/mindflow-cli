// Send a query request off to the Mindflow server to get a response 
// from GPT model using a prompt and set of reference hashes.

use std::process;

use serde::{Deserialize, Serialize};
use reqwest::{Client};

use crate::utils::config::{CONFIG};

use super::status::http_status::HttpStatus;

#[derive(Serialize)]
pub(crate) struct QueryRequest {
    pub(crate) query_text: String,
    pub(crate) reference_hashes: Vec<String>,
    pub(crate) auth: String
}

impl QueryRequest {
    pub fn new(query_text: String, reference_hashes: Vec<String>) -> QueryRequest {
        QueryRequest {
            query_text,
            reference_hashes,
            auth: CONFIG.get_auth_token()
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct QueryResponse {
    pub(crate) text: String,
}

// Send a query request off to the Mindflow server to get a response.
pub(crate) async fn request_query(client:&Client, query_text: String, processed_hashes: Vec<String>) -> QueryResponse {
    let query = QueryRequest::new(query_text, processed_hashes);
    let res = match client
        .post(&format!("{}/query", CONFIG.get_api_location()))
        .json(&query)
        .send()
        .await {
            Ok(res) => res,
            Err(e) => {
                println!("Error: Could not send query request: {}", e);
                process::exit(1);
            }
        };
    
    // match status code
    let status = match res.status().as_u16() {
        200 => HttpStatus::Ok,
        400 => HttpStatus::BadRequest,
        401 => HttpStatus::Unauthorized,
        _   => HttpStatus::InternalServerError
    };

    // match response
    match status {
        HttpStatus::Ok => {
            match res.json().await {
                Ok(query_response) => query_response,
                Err(e) => {
                    println!("Error: Could not get query response: {}", e);
                    process::exit(1);
                }
            }
        }
        HttpStatus::BadRequest => {
            println!("Error: Bad Request.");
            process::exit(1);
        }
        HttpStatus::Unauthorized => {
            println!("Invalid authorization token.");
            process::exit(1);
        }
        HttpStatus::InternalServerError => {
            println!("Error: Could not get query response");
            process::exit(1);
        }
    }
}
