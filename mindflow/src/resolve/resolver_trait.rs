use std::{collections::HashMap};

use crate::utils::reference::Reference;

pub(crate) trait Resolver {
    fn should_resolve(&self, path_string: &String) -> bool;
    fn resolve(&self, path_string: &String) -> HashMap<String, Reference>;
}
