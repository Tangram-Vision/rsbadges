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

use rsbadges::{Badge, Style};
use std::fs;
use std::path::Path;

pub fn save_svg_to_tmp(filename: &str, svg: String) {
    let ci_path = std::env::temp_dir();
    let svg_path = ci_path.join(Path::new(filename));
    println!("Saving badge to {:#?}", svg_path);
    if let Err(c) = fs::write(svg_path, svg) {
        println!("ERROR: Could not save badge: {}", c);
    }
}

#[test]
fn create_badges_with_all_fields_populated() {
    let badge = Badge {
        label_text: String::from("version"),
        msg_text: String::from("1.2.3"),
        label_color: String::from("#555"),
        msg_color: String::from("#007ec6"),
        ..Badge::default()
    };
    let mut svg = match Style::Flat(badge.clone()).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge.svg", svg);
    svg = match Style::FlatSquare(badge.clone()).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_square_badge.svg", svg);
    svg = match Style::Plastic(badge.clone()).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("plastic_badge.svg", svg);
    // For comparison: https://img.shields.io/static/v1?label=version&message=1.2.3&style=for-the-badge
    svg = match Style::ForTheBadge(badge.clone()).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("for_the_badge.svg", svg);
    svg = match Style::Social(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("social.svg", svg);
}

#[test]
fn create_badge_with_logo() {
    let badge = Badge {
        label_text: String::from("version"),
        msg_text: String::from("1.2.3"),
        logo: String::from("https://simpleicons.org/icons/rust.svg"),
        ..Badge::default()
    };
    let svg = match Style::Flat(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge_logo.svg", svg);
}

#[test]
fn create_badge_with_logo_local() {
    let path = std::env::current_dir().unwrap();
    let font_path = path.join(Path::new("tests/rust.svg"));
    let badge = Badge {
        label_text: String::from("version"),
        msg_text: String::from("1.2.3"),
        logo: String::from(font_path.to_str().unwrap()),
        ..Badge::default()
    };
    let svg = match Style::Flat(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge_logo_local.svg", svg);
}

#[test]
fn create_badge_embed_logo() {
    let badge = Badge {
        logo: String::from("https://simpleicons.org/icons/rust.svg"),
        embed_logo: true,
        ..Badge::default()
    };
    let mut svg = match Style::Flat(badge.clone()).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge_logo_embedded.svg", svg);
    // For comparison: https://img.shields.io/static/v1?label=test&message=test&style=for-the-badge&logo=rust
    svg = match Style::ForTheBadge(badge.clone()).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("for_the_badge_logo_embedded.svg", svg);
    // For comparison: https://img.shields.io/static/v1?label=version&message=1.2.3&style=social
    svg = match Style::Social(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("social_logo_embedded.svg", svg);
}

#[test]
fn create_badge_embed_logo_local_src() {
    let path = std::env::current_dir().unwrap();
    let font_path = path.join(Path::new("tests/rust.svg"));
    let badge = Badge {
        logo: String::from(font_path.to_str().unwrap()),
        embed_logo: true,
        ..Badge::default()
    };
    let svg = match Style::Flat(badge).generate_svg() {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            unreachable!()
        }
    };
    save_svg_to_tmp("flat_badge_logo_local_embedded.svg", svg);
}

#[test]
fn create_badge_chinese_characters() {
    let badge = Badge {
        label_text: String::from("版"),
        msg_text: String::from("不知道"),
        ..Badge::default()
    };
    let svg = match Style::Flat(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge_chinese.svg", svg);
}

#[test]
fn create_badge_arabic_characters() {
    let badge = Badge {
        label_text: String::from("اختبار"),
        msg_text: String::from("انا لا اعرف"),
        ..Badge::default()
    };
    let svg = match Style::Flat(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge_arabic.svg", svg);
}

#[test]
fn create_badge_metal() {
    let badge = Badge {
        label_text: String::from("röck döts"),
        msg_text: String::from("very metal indeed"),
        ..Badge::default()
    };
    let svg = match Style::Flat(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge_metal.svg", svg);
}

#[test]
fn create_badge_badge_link() {
    let badge = Badge {
        badge_link: String::from("http://www.crates.io"),
        ..Badge::default()
    };
    let svg = match Style::Flat(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge_full_link.svg", svg);
}

#[test]
fn create_badge_with_separate_label_msg_links() {
    let badge = Badge {
        label_link: String::from("http://www.tangramvision.com"),
        msg_link: String::from("http://www.crates.io"),
        ..Badge::default()
    };
    let svg = match Style::Flat(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge_two_links.svg", svg);
}
