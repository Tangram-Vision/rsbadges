// Copyright 2021 RSBadges authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::PathBuf;

pub struct Badge {
    left_text: String,
    right_text: String,
    link: String,
    left_link: String,
    right_link: String,
    left_color: css_color::Rgba,
    right_color: css_color::Rgba,
    logo: PathBuf,
    embed_logo: bool,
    title: String,
    left_title: String,
    right_title: String,
    browser: bool,
}

#[cfg(test)]
mod tests {}
