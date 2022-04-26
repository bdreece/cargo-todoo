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

use std::io::Result;
use std::path::PathBuf;

use lazy_static::lazy_static;
use regex::Regex;
use tokio::{
    fs,
    io::{AsyncBufReadExt, BufReader},
};

pub struct Comment {
    pub file_name: String,
    pub priority: usize,
    pub message: String,
}

// TODOO: impl Comment

pub struct File {
    pub todos: Vec<Comment>,
    pub fixmes: Vec<Comment>,
}

impl File {
    pub fn new() -> Self {
        Self {
            todos: vec![],
            fixmes: vec![],
        }
    }

    pub async fn parse_todos(&mut self, path: &PathBuf) -> Result<()> {
        lazy_static! {
            static ref TODO_REGEX: Regex =
                Regex::new(r"^(?:.*)TOD(O+):\s([^\*/]*)(?:\**/+)*$").unwrap();
        }

        // FIXME: Unwrapping on file_name
        let file_name = path.file_name().unwrap();
        let data = fs::read(path.as_path()).await?;
        let reader = BufReader::new(data.as_slice());
        let mut lines = reader.lines();

        loop {
            match lines.next_line().await? {
                Some(line) => {
                    if let Some(captures) = TODO_REGEX.captures(line.as_str()) {
                        if let Some(priority) = captures.get(1) {
                            if let Some(message) = captures.get(2) {
                                self.todos.push(Comment {
                                    // FIXMEE: Still unwrapping on file_name
                                    file_name: file_name.to_str().unwrap().to_string(),
                                    priority: priority.as_str().len(),
                                    message: String::from(message.as_str()),
                                });
                            }
                        }
                    }
                }
                None => break,
            }
        }

        Ok(())
    }

    pub async fn parse_fixmes(&mut self, path: &PathBuf) -> Result<()> {
        lazy_static! {
            static ref FIXME_REGEX: Regex =
                Regex::new(r"^(?:.*)FIXM(E+):\s([^\*/]*)(?:\**/+)*$").unwrap();
        }

        let file_name = path.file_name().unwrap();
        let data = fs::read(path.as_path()).await?;
        let reader = BufReader::new(data.as_slice());
        let mut lines = reader.lines();

        loop {
            match lines.next_line().await? {
                Some(line) => {
                    if let Some(captures) = FIXME_REGEX.captures(line.as_str()) {
                        if let Some(priority) = captures.get(1) {
                            if let Some(message) = captures.get(2) {
                                self.fixmes.push(Comment {
                                    file_name: file_name.to_str().unwrap().to_string(),
                                    priority: priority.as_str().len(),
                                    message: String::from(message.as_str()),
                                });
                            }
                        }
                    }
                }
                None => break,
            }
        }

        Ok(())
    }
}
