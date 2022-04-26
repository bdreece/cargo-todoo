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

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    name = "cargo_todoo",
    version = "0.1",
    author = "Brian Reece",
    about = "A todo comment aggregator for Cargo"
)]
pub struct Args {
    #[clap(short, long, parse(from_occurrences), help = "Enable verbose output")]
    pub verbose: usize,

    #[clap(short, long, help = "Ignore comments matching regex string")]
    pub ignore_regex: Option<String>,

    #[clap(short, long, help = "Skip source files matching regex string")]
    pub skip_regex: Option<String>,

    #[clap(parse(from_os_str), default_value("."), help = "Path to crate root")]
    pub path: PathBuf,
    // TODOO: Add additional CLI flags
}
