use std::collections::LinkedList;
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;

use async_recursion::async_recursion;
use regex::Regex;
use tokio::fs;

pub struct Tree {
    pub entries: LinkedList<PathBuf>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            entries: LinkedList::new(),
        }
    }

    #[async_recursion]
    async fn dfs(root: &PathBuf, ignore: &Option<String>) -> Result<LinkedList<PathBuf>> {
        let mut sources: LinkedList<PathBuf> = LinkedList::new();
        let mut entries = fs::read_dir(&root).await?;
        let ignore_regex =
            match ignore {
                Some(ignore) => Some(Regex::new(ignore.as_str()).map_err(|_| {
                    Error::new(ErrorKind::InvalidInput, "invalid ignore regex string")
                })?),
                None => None,
            };

        loop {
            match entries.next_entry().await? {
                Some(entry) => {
                    let file_name = entry.file_name();
                    if let Some(ignore_regex) = &ignore_regex {
                        if ignore_regex.is_match(file_name.to_str().unwrap()) {
                            continue;
                        }
                    }
                    let file_type = entry.file_type().await?;
                    if file_type.is_dir() {
                        let path = entry.path();
                        let entry_sources = Tree::dfs(&path, ignore);
                        sources.append(&mut entry_sources.await?);
                    } else if file_type.is_file() {
                        sources.push_back(entry.path());
                    }
                }
                None => break,
            }
        }

        Ok(sources)
    }

    pub async fn traverse(&mut self, root: &PathBuf, ignore: &Option<String>) -> Result<()> {
        let mut tree = Tree::dfs(root, ignore).await?;
        self.entries.append(&mut tree);
        Ok(())
    }
}
