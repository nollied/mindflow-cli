use std::collections::HashMap;
use std::fs;
use std::path::Path;


use sha2::{Digest, Sha256};

use crate::utils::reference::Reference;
use crate::utils::git::{get_git_files, is_within_git_repo};
use crate::resolve::resolver_trait::Resolver;


pub(crate) struct PathResolver {}

impl PathResolver {
    pub fn extract_files(&self, path: &Path) -> Vec<String> {
        if path.is_dir() {
            if is_within_git_repo(path) {
                return get_git_files(path)
            } else {
                panic!("Path is not within a git repository")
            }
        } else {
            vec![path.to_string_lossy().to_string()] 
        }
    }

    fn resolve_file(&self, file: &str) -> HashMap<String, Reference> {
        let file_bytes = fs::read(file).unwrap();
        let file_text = std::str::from_utf8(&file_bytes);
        match file_text {
            Ok(_) => {
                let mut hasher = Sha256::new();
                hasher.update(&file_bytes);
                let file_hash = hasher.finalize();
                let file_hash_string = format!("{:x}", file_hash);
                let mut map = HashMap::new();
                map.insert(
                    file_hash_string.clone(),
                    Reference::new(
                        file_hash_string,
                        std::str::from_utf8(&file_bytes).unwrap().to_string(),
                        "path".to_string(),
                        file.to_string(),
                    ),
                );
                map
            }
            Err(_) => {
                return HashMap::new();
            }
        }
    }
}

impl Resolver for PathResolver {
    fn should_resolve(&self, path_string: &String) -> bool {
        let path = Path::new(path_string);
        path.is_dir() || path.is_file()
    }

    fn resolve(&self, path_string: &String) -> HashMap<String, Reference> {
        let mut resolved_files = HashMap::new();
        for file in self.extract_files(&Path::new(&path_string)) {
            resolved_files.extend(self.resolve_file(&file));
        }

        resolved_files
    }
}
