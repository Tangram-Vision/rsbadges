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

use askama::Template;
use rand::{distributions::Alphanumeric, Rng};
use rusttype::{point, Font, Scale};
use std::env;
use std::path::{Path, PathBuf}; // bring trait in scope // 0.8

// Keep these grouped...
#[derive(Template, Debug)]
#[template(path = "badge_template_flat.xml.j2", escape = "xml")]
struct BadgeTemplate<'a> {
    left_text: &'a str,
    right_text: &'a str,
    full_link: &'a str,
    left_link: &'a str,
    right_link: &'a str,
    left_color: &'a str,
    right_color: &'a str,
    logo: &'a str,
    full_title: &'a str,
    left_title: &'a str,
    right_title: &'a str,
    logo_width: usize,
    logo_padding: usize,
    left_text_width: usize,
    right_text_width: usize,
    left_text_x: usize,
    right_text_x: usize,
    left_width: usize,
    right_width: usize,
    id_smooth: &'a str,
    id_round: &'a str,
}

// Docs: https://gitlab.redox-os.org/redox-os/rusttype/-/blob/master/dev/examples/ascii.rs
fn load_font<'a>() -> Font<'a> {
    // TODO: Deal with this
    let font_data = include_bytes!("../fonts/DejaVuSans.ttf");
    Font::try_from_bytes(font_data as &[u8]).expect("error constructing a Font from bytes")
}

fn get_text_dims(font: &Font, text: &str, font_size: f32) -> (f32, f32) {
    let scale = Scale::uniform(font_size);
    let layout = font.layout(text, scale, point(0.0, 0.0));
    let glyphs_width = layout.fold(0.0, |acc, x| {
        acc + x.into_unpositioned().h_metrics().advance_width
    });
    let v_metrics = font.v_metrics(scale);
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil();
    (glyphs_width, glyphs_height)
}

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

impl Default for Badge {
    fn default() -> Badge {
        Badge::new(
            String::from("test"),
            String::from("test"),
            "white".parse().unwrap(),
            "green".parse().unwrap(),
        )
    }
}

fn color_to_string(color: css_color::Rgba) -> String {
    format!(
        "rgb({}, {}, {})",
        color.red * 255.0,
        color.green * 255.0,
        color.blue * 255.0
    )
}

impl Badge {
    pub fn new(
        left_text: String,
        right_text: String,
        left_color: css_color::Rgba,
        right_color: css_color::Rgba,
    ) -> Badge {
        Badge {
            left_text,
            right_text,
            link: String::from(""),
            left_link: String::from(""),
            right_link: String::from(""),
            left_color,
            right_color,
            logo: PathBuf::default(),
            embed_logo: false,
            title: String::from(""),
            left_title: String::from(""),
            right_title: String::from(""),
            browser: false,
        }
    }

    pub fn to_flat_badge(&self) -> String {
        let id_suffix: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        let font = load_font();
        let left_text_width = get_text_dims(&font, &self.left_text, 110.0).0;
        let right_text_width = get_text_dims(&font, &self.right_text, 110.0).0;
        let mut logo_padding = 0.0;
        // TODO: Get logo width
        let logo_width = 0.0;
        if self.logo.to_str().is_some() && self.left_text.is_empty() {
            logo_padding = 3.0;
        }

        let left_color = color_to_string(self.left_color);
        let right_color = color_to_string(self.right_color);

        // These factors are scaled down by 10x on the svg side, by design.
        let mut ltw_scaled = left_text_width / 10.0;
        let mut rtw_scaled = right_text_width / 10.0;
        ltw_scaled += self.left_text.len() as f32;
        rtw_scaled += self.right_text.len() as f32;
        let left_width = ltw_scaled + logo_width + logo_padding + 10.0;
        let right_width = rtw_scaled + 10.0;
        let left_text_x = ((left_width / 2.0) + logo_width + logo_padding + 1.0) as usize * 10;
        let right_text_x = (left_width + (right_width as f32 / 2.0) - 1.0) as usize * 10;

        let id_smooth = format!("smooth{}", id_suffix);
        let id_round = format!("round{}", id_suffix);
        let logo_url = self.logo.to_str().unwrap();

        let flat_badge = BadgeTemplate {
            left_text: &self.left_text,
            right_text: &self.right_text,
            full_link: &self.link,
            left_link: &self.left_link,
            right_link: &self.right_link,
            left_color: &left_color,
            right_color: &right_color,
            logo: logo_url,
            full_title: &self.title,
            left_title: &self.left_title,
            right_title: &self.right_title,
            logo_width: logo_width as usize,
            logo_padding: logo_padding as usize,
            left_text_width: ltw_scaled as usize * 10,
            right_text_width: rtw_scaled as usize * 10,
            left_text_x,
            right_text_x,
            left_width: left_width as usize,
            right_width: right_width as usize,
            id_smooth: &id_smooth,
            id_round: &id_round,
        };
        println!("{:#?}", flat_badge);

        flat_badge.render().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use crate::badge::Badge;
    use css_color::Rgba;
    use std::fs;
    use std::path::Path;
    #[test]
    fn create_flat_badge() {
        let left_text = String::from("version");
        let right_text = String::from("1.2.3");
        let left_color: Rgba = "#555".parse().unwrap();
        let right_color: Rgba = "#007ec6".parse().unwrap();
        let badge = Badge::new(left_text, right_text, left_color, right_color);

        let ci_path = std::env::current_dir().unwrap();

        let commit_svg_path = ci_path.join(Path::new("badge_commits.svg"));
        println!("Saving commit badge to {:#?}", commit_svg_path);
        if let Err(c) = fs::write(commit_svg_path, badge.to_flat_badge()) {
            println!("ERROR: Could not save badge_commits: {}", c);
        }
    }
}
