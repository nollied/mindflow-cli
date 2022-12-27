use clap::{Parser, ArgAction};

use crate::resolve::resolve::{resolve};
use crate::requests::index_references::request_index_references;
use crate::requests::unindexed_references::{request_unindexed_references, UnindexedReferencesResponse};
use crate::requests::query::{request_query, QueryResponse};

#[derive(Parser)]
pub(crate) struct Query {
    #[arg(index = 1)]
    pub(crate) query: String,
    #[arg(index = 2)]
    pub(crate) references: Vec<String>,
    #[arg(short = 's', long = "skip-response", action = ArgAction::SetTrue, value_name = "Skip response from GPT model.")]
    pub(crate) skip_response: bool,
    #[arg(short = 't', long = "clipboard", action = ArgAction::SetTrue, value_name = "Copy response to clipboard.")]
    pub(crate) clipboard: bool,
}

impl Query {
    pub(crate) async fn execute(&mut self) {
        // create query request handler
        let client = reqwest::Client::new();

        let resolved_references = resolve(&self.references);
        let unindexed_references_response = request_unindexed_references(&client, &resolved_references).await;
        let unindexed_reference_response: UnindexedReferencesResponse = match unindexed_references_response {
            Ok(response) => {
                response
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        };

        request_index_references(&client, &resolved_references, &unindexed_reference_response.unindexed_hashes).await;
        let request_query_response = request_query(&client, self.query.clone(), &resolved_references).await;
        let query_response: QueryResponse = match request_query_response {
            Ok(response) => {
                response
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        };
        if !self.skip_response {
            //println!("{}", query_response.text);
        }
    }
}
