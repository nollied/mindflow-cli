use std::collections::HashMap;

use crate::resolve::path_resolver::PathResolver;
use crate::resolve::resolver_trait::Resolver;
use crate::utils::reference::Reference;

pub(crate) fn resolve(references: &Vec<String>) -> HashMap<String, Reference> {
    let mut resolved_references = HashMap::new();
    for reference in references {
        let resolvers = [PathResolver{}];
        let mut resolved = false;
        for resolver in resolvers.iter() {
            if resolver.should_resolve(reference) {
                resolved = true;
                for (k, v) in resolver.resolve(reference) {
                    resolved_references.insert(k, v);
                }
            }
        }
        if !resolved {
            println!("Could not resolve reference: {}", reference);
        }
    }
    resolved_references
}