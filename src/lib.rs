// Copyright (c) 2021 RSBadges Authors, All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
// 1. Redistributions of source code must retain the above copyright notice,
//    this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
// 3. Neither the name of the copyright holder nor the names of its contributors
//    may be used to endorse or promote products derived from this software
//    without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
// IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
// INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
// BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA,
// OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY
// OF SUCH DAMAGE.

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
