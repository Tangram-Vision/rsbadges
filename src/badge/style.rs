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

use super::badge_generation;
use super::badge_layout;
use super::badge_type::*;

#[derive(Debug)]
pub enum Style {
    Plastic(Badge),
    Flat(Badge),
    FlatSquare(Badge),
    ForTheBadge(Badge),
}

impl Style {
    pub fn generate_svg(&self) -> String {
        let layout = match self {
            Style::Flat(badge) => badge_layout::flat_or_square(badge),
            Style::FlatSquare(badge) => badge_layout::flat_or_square(badge),
            Style::Plastic(badge) => badge_layout::plastic(badge),
            _ => Layout::default(),
        };

        match self {
            Style::Flat(badge) => badge_generation::flat_svg(badge, layout),
            Style::FlatSquare(badge) => badge_generation::flat_square_svg(badge, layout),
            Style::Plastic(badge) => badge_generation::plastic_svg(badge, layout),
            _ => String::from("Not implemented yet!"),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::badge::Badge;
    use crate::badge::Style;
    use std::fs;
    use std::path::Path;

    fn save_badge_to_tmp(filename: &str, svg: String) {
        let ci_path = std::env::temp_dir();
        let svg_path = ci_path.join(Path::new(filename));
        println!("Saving badge to {:#?}", svg_path);
        if let Err(c) = fs::write(svg_path, svg) {
            println!("ERROR: Could not save badge: {}", c);
        }
    }

    #[test]
    fn create_badges() {
        let badge = Badge {
            label_text: String::from("version"),
            msg_text: String::from("1.2.3"),
            label_color: "#555".parse().unwrap(),
            msg_color: "#007ec6".parse().unwrap(),
            ..Default::default()
        };

        save_badge_to_tmp("flat_badge.svg", Style::Flat(badge.clone()).generate_svg());
        save_badge_to_tmp(
            "flat_square_badge.svg",
            Style::FlatSquare(badge.clone()).generate_svg(),
        );
        save_badge_to_tmp("plastic_badge.svg", Style::Plastic(badge).generate_svg());
    }
}

//     #[test]
//     fn create_badges_with_logos() {
//         let mut badge = Badge {
//             label_text: String::from("version"),
//             msg_text: String::from("1.2.3"),
//             label_color: "#555".parse().unwrap(),
//             msg_color: "#007ec6".parse().unwrap(),
//             logo: String::from("https://simpleicons.org/icons/slack.svg"),
//             ..Default::default()
//         };

//         save_badge_to_tmp("flat_logo.svg", &mut badge, Flavor::Flat);
//         save_badge_to_tmp("flat_square_logo.svg", &mut badge, Flavor::FlatSquare);
//         save_badge_to_tmp("plastic_logo.svg", &mut badge, Flavor::Plastic);
//     }

//     #[test]
//     fn create_badge_chinese_characters() {
//         let mut badge = Badge {
//             label_text: String::from("版"),
//             msg_text: String::from("不知道"),
//             label_color: "#555".parse().unwrap(),
//             msg_color: "#007ec6".parse().unwrap(),
//             ..Default::default()
//         };

//         save_badge_to_tmp("flat_chinese.svg", &mut badge, Flavor::Flat);
//         save_badge_to_tmp("flat_square_chinese.svg", &mut badge, Flavor::FlatSquare);
//         save_badge_to_tmp("plastic_chinese.svg", &mut badge, Flavor::Plastic);
//     }

//     #[test]
//     fn create_badge_arabic_characters() {
//         let mut badge = Badge {
//             label_text: String::from("اختبار"),
//             msg_text: String::from("انا لا اعرف"),
//             label_color: "#555".parse().unwrap(),
//             msg_color: "#007ec6".parse().unwrap(),
//             ..Default::default()
//         };

//         save_badge_to_tmp("flat_arabic.svg", &mut badge, Flavor::Flat);
//         save_badge_to_tmp("flat_square_arabic.svg", &mut badge, Flavor::FlatSquare);
//         save_badge_to_tmp("plastic_arabic.svg", &mut badge, Flavor::Plastic);
//     }

//     #[test]
//     fn create_badge_metal() {
//         let mut badge = Badge {
//             label_text: String::from("röck döts"),
//             msg_text: String::from("metal"),
//             label_color: "#555".parse().unwrap(),
//             msg_color: "#007ec6".parse().unwrap(),
//             ..Default::default()
//         };

//         save_badge_to_tmp("flat_metal.svg", &mut badge, Flavor::Flat);
//         save_badge_to_tmp("flat_square_metal.svg", &mut badge, Flavor::FlatSquare);
//         save_badge_to_tmp("plastic_metal.svg", &mut badge, Flavor::Plastic);
//     }

//     #[test]
//     fn create_badges_badge_link() {
//         let mut badge = Badge {
//             label_text: String::from("version"),
//             msg_text: String::from("1.2.3"),
//             label_color: "#555".parse().unwrap(),
//             msg_color: "#007ec6".parse().unwrap(),
//             badge_link: String::from("http://www.tangramvision.com"),
//             ..Default::default()
//         };

//         save_badge_to_tmp("flat_link.svg", &mut badge, Flavor::Flat);
//         save_badge_to_tmp("flat_square_link.svg", &mut badge, Flavor::FlatSquare);
//         save_badge_to_tmp("plastic_link.svg", &mut badge, Flavor::Plastic);
//     }

//     #[test]
//     fn create_badges_with_label_msg_links() {
//         let mut badge = Badge {
//             label_text: String::from("version"),
//             msg_text: String::from("1.2.3"),
//             label_color: "#555".parse().unwrap(),
//             msg_color: "#007ec6".parse().unwrap(),
//             label_link: String::from("http://www.tangramvision.com"),
//             msg_link: String::from("http://www.google.com"),
//             ..Default::default()
//         };

//         save_badge_to_tmp("flat_links.svg", &mut badge, Flavor::Flat);
//         save_badge_to_tmp("flat_square_links.svg", &mut badge, Flavor::FlatSquare);
//         save_badge_to_tmp("plastic_links.svg", &mut badge, Flavor::Plastic);
//     }
// }
