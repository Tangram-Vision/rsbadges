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

mod badge;

use badge::Badge;

#[cfg(test)]
mod tests {

    use crate::badge::Badge;
    use css_color::Rgba;
    use std::fs;
    use std::path::Path;
    #[test]
    fn create_flat_badge() {
        let left_text = String::from("test");
        let right_text = String::from("test");
        let left_color: Rgba = "#555".parse().unwrap();
        let right_color: Rgba = "#007ec6".parse().unwrap();
        let badge = Badge::new(left_text, right_text, left_color, right_color);

        let ci_path = std::env::current_dir().unwrap();

        let commit_svg_path = ci_path.join(Path::new("badge_commits.svg"));
        println!("Saving commit badge to {:#?}", commit_svg_path);
        if let Err(c) = fs::write(commit_svg_path, badge.to_flat_badge()) {
            println!("ERROR: Could not save badge_commits: {}", c);
        }
        // println!("{}", badge.to_plastic_badge());
    }
}
