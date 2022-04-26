/*
 * MIT License
 *
 * Copyright (c) 2022 Brian Reece
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

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
                    let path = entry.path();
                    let file_name = entry.file_name();
                    let file_type = entry.file_type().await?;
                    let file_extension = path.extension();

                    if let Some(ignore_regex) = &ignore_regex {
                        if ignore_regex.is_match(file_name.to_string_lossy().as_ref()) {
                            continue;
                        }
                    }
                    if file_type.is_dir() {
                        let entry_sources = Tree::dfs(&path, ignore);
                        sources.append(&mut entry_sources.await?);
                    } else if file_type.is_file() {
                        if let Some(extension) = file_extension {
                            if extension == "rs" {
                                sources.push_back(path);
                            }
                        }
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
