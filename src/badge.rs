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

fn get_text_dims(font: &Font, text: &str) -> (usize, usize) {
    // Desired font pixel height
    let height: f32 = 8.0; // to get 80 chars across (fits most terminals); adjust as desired
    let pixel_height = height.ceil() as usize;

    // 2x scale in x direction to counter the aspect ratio of monospace characters.
    let scale = Scale {
        x: height * 1.5,
        y: height,
    };

    // The origin of a line of text is at the baseline (roughly where
    // non-descending letters sit). We don't want to clip the text, so we shift
    // it down with an offset when laying it out. v_metrics.ascent is the
    // distance between the baseline and the highest edge of any glyph in
    // the font. That's enough to guarantee that there's no clipping.
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);
    let glyphs: Vec<_> = font.layout(&text, scale, offset).collect();

    // Find the most visually pleasing width to display
    let width = glyphs
        .iter()
        .rev()
        .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
        .next()
        .unwrap_or(0.0)
        .ceil() as usize;

    (width, pixel_height)
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
        let left_text_width = get_text_dims(&font, &self.left_text).0;
        let right_text_width = get_text_dims(&font, &self.right_text).0;
        let mut logo_padding = 0;
        if self.logo.to_str().is_some() && self.left_text.is_empty() {
            logo_padding = 3;
        }

        let badge_d = BadgeDerived {
            id_suffix,
            logo_width: 0,
            logo_padding,
            left_text_width,
            right_text_width,
            left_color: color_to_string(self.left_color),
            right_color: color_to_string(self.right_color),
        };

        let left_width = badge_d.left_text_width + 10 + badge_d.logo_width + badge_d.logo_padding;
        let right_width = badge_d.right_text_width + 10;
        let id_smooth = format!("smooth{}", badge_d.id_suffix);
        let id_round = format!("round{}", badge_d.id_suffix);
        let logo_url = self.logo.to_str().unwrap();

        let flat_badge = BadgeTemplate {
            left_text: &self.left_text,
            right_text: &self.right_text,
            full_link: &self.link,
            left_link: &self.left_link,
            right_link: &self.right_link,
            left_color: &badge_d.left_color,
            right_color: &badge_d.right_color,
            logo: logo_url,
            full_title: &self.title,
            left_title: &self.left_title,
            right_title: &self.right_title,
            logo_width: badge_d.logo_width,
            logo_padding: badge_d.logo_padding,
            left_width,
            right_width,
            id_smooth: &id_smooth,
            id_round: &id_round,
        };

        flat_badge.render().unwrap()
    }
}

struct BadgeDerived {
    id_suffix: String,
    logo_width: usize,
    logo_padding: usize,
    left_text_width: usize,
    right_text_width: usize,
    left_color: String,
    right_color: String,
}

fn color_to_string(color: css_color::Rgba) -> String {
    format!(
        "rgb({}, {}, {})",
        color.red * 255.0,
        color.green * 255.0,
        color.blue * 255.0
    )
}

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
    }
}
