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
#[template(path = "badge_template_plastic.xml.j2", escape = "xml")]
struct BadgeTemplate<'a> {
    label_text: &'a str,
    msg_text: &'a str,
    badge_link: &'a str,
    label_link: &'a str,
    msg_link: &'a str,
    label_color: &'a str,
    msg_color: &'a str,
    logo: &'a str,
    full_badge_title: &'a str,
    label_title: &'a str,
    msg_title: &'a str,
    logo_width: usize,
    logo_padding: usize,
    label_text_width: usize,
    msg_text_width: usize,
    label_text_x: usize,
    msg_text_x: usize,
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

fn get_text_dims(font: &Font, text: &str, font_size: f32, kerning_pix: f32) -> (f32, f32) {
    let scale = Scale::uniform(font_size);
    let layout = font.layout(text, scale, point(0.0, 0.0));
    let mut glyphs_width = layout.fold(0.0, |acc, x| {
        acc + x.into_unpositioned().h_metrics().advance_width
    });
    let v_metrics = font.v_metrics(scale);
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil();
    // 1px padding
    glyphs_width += text.len() as f32 * kerning_pix;
    if glyphs_width as usize % 2 == 0 {
        glyphs_width += 1.0;
    }
    (glyphs_width, glyphs_height)
}

pub struct Badge {
    pub label_text: String,
    pub msg_text: String,
    pub badge_link: String,
    pub label_link: String,
    pub msg_link: String,
    pub label_color: css_color::Rgba,
    pub msg_color: css_color::Rgba,
    pub logo: String,
    pub embed_logo: bool,
    pub badge_title: String,
    pub label_title: String,
    pub msg_title: String,
    pub open_in_browser: bool,
    derived_info: DerivedInfo,
}

impl<'a> Default for Badge {
    fn default() -> Badge {
        Badge {
            label_text: String::from("test"),
            msg_text: String::from("test"),
            badge_link: String::from(""),
            label_link: String::from(""),
            msg_link: String::from(""),
            label_color: "#555".parse().unwrap(),
            msg_color: "#007ec6".parse().unwrap(),
            logo: String::from(""),
            embed_logo: false,
            badge_title: String::from(""),
            label_title: String::from(""),
            msg_title: String::from(""),
            open_in_browser: false,
            derived_info: DerivedInfo::default(),
        }
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

#[derive(Default)]
struct DerivedInfo {
    label_text_width: f32,
    msg_text_width: f32,
    label_total_width: f32,
    msg_total_width: f32,
    label_text_x: f32,
    msg_text_x: f32,
    logo_padding: f32,
    label_color: String,
    msg_color: String,
    logo_width: f32,
}

impl Badge {
    fn derive_construction_info(&mut self) {
        let mut derived_info = DerivedInfo::default();

        let font = load_font();
        derived_info.label_text_width = get_text_dims(&font, &self.label_text, 11.0, 0.8).0;
        derived_info.msg_text_width = get_text_dims(&font, &self.msg_text, 11.0, 0.8).0;

        // Padding and spacing calculations
        derived_info.logo_padding = 0.0;
        // TODO: Get logo width
        derived_info.logo_width = 0.0;
        if !self.logo.is_empty() && self.label_text.is_empty() {
            derived_info.logo_padding = 3.0;
        }

        let horiz_padding = 5.0;
        let label_text_margin = derived_info.logo_width + 1.0;
        derived_info.label_total_width = 0.0;
        if !self.label_text.is_empty() {
            derived_info.label_total_width =
                derived_info.label_text_width + (2.0 * horiz_padding) + derived_info.logo_width;
        }
        let mut msg_text_margin = derived_info.label_total_width;
        if !self.msg_text.is_empty() {
            msg_text_margin -= 1.0;
        }

        if self.label_text.is_empty() {
            if !self.logo.is_empty() {
                msg_text_margin += derived_info.logo_width + horiz_padding;
            } else {
                msg_text_margin += 1.0;
            }
        }

        derived_info.msg_total_width = derived_info.msg_text_width + (2.0 * horiz_padding);
        if !self.logo.is_empty() && self.label_text.is_empty() {
            derived_info.msg_total_width += derived_info.logo_width + horiz_padding - 1.0;
        }

        derived_info.label_text_x =
            label_text_margin + (0.5 * derived_info.label_text_width) + horiz_padding;
        derived_info.msg_text_x =
            msg_text_margin + (0.5 * derived_info.msg_text_width) + horiz_padding;

        // Scale back up for the SVG
        derived_info.label_text_width *= 10.0;
        derived_info.msg_text_width *= 10.0;
        derived_info.label_text_x *= 10.0;
        derived_info.msg_text_x *= 10.0;

        // Color conversion to string
        derived_info.label_color = color_to_string(self.label_color);
        derived_info.msg_color = color_to_string(self.msg_color);

        self.derived_info = derived_info;
    }

    pub fn generate_plastic_svg(&mut self) -> String {
        self.derive_construction_info();
        let id_suffix: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let id_smooth = format!("smooth{}", id_suffix);
        let id_round = format!("round{}", id_suffix);
        let plastic_badge = BadgeTemplate {
            label_text: &self.label_text,
            msg_text: &self.msg_text,
            badge_link: &self.badge_link,
            label_link: &self.label_link,
            msg_link: &self.msg_link,
            label_color: &self.derived_info.label_color,
            msg_color: &self.derived_info.msg_color,
            logo: &self.logo,
            full_badge_title: &self.badge_title,
            label_title: &self.label_title,
            msg_title: &self.msg_title,
            logo_width: self.derived_info.logo_width as usize,
            logo_padding: self.derived_info.logo_padding as usize,
            label_text_width: self.derived_info.label_text_width as usize,
            msg_text_width: self.derived_info.msg_text_width as usize,
            label_text_x: self.derived_info.label_text_x as usize,
            msg_text_x: self.derived_info.msg_text_x as usize,
            left_width: self.derived_info.label_total_width as usize,
            right_width: self.derived_info.msg_total_width as usize,
            id_smooth: &id_smooth,
            id_round: &id_round,
        };

        plastic_badge.render().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use crate::badge::Badge;
    use std::fs;
    use std::path::Path;
    #[test]
    fn create_plastic_badge() {
        let mut badge = Badge {
            label_text: String::from("version"),
            msg_text: String::from("1.2.3"),
            label_color: "#555".parse().unwrap(),
            msg_color: "#007ec6".parse().unwrap(),
            ..Default::default()
        };

        let ci_path = std::env::current_dir().unwrap();

        let svg_path = ci_path.join(Path::new("plastic_badge.svg"));
        println!("Saving badge to {:#?}", svg_path);
        if let Err(c) = fs::write(svg_path, badge.generate_plastic_svg()) {
            println!("ERROR: Could not save badge_commits: {}", c);
        }
    }

    #[test]
    fn create_plastic_badge_with_badge_link() {
        let mut badge = Badge {
            label_text: String::from("version"),
            msg_text: String::from("1.2.3"),
            label_color: "#555".parse().unwrap(),
            msg_color: "#007ec6".parse().unwrap(),
            badge_link: String::from("http://www.tangramvision.com"),
            ..Default::default()
        };

        let ci_path = std::env::current_dir().unwrap();

        let svg_path = ci_path.join(Path::new("plastic_badge_link.svg"));
        println!("Saving badge to {:#?}", svg_path);
        if let Err(c) = fs::write(svg_path, badge.generate_plastic_svg()) {
            println!("ERROR: Could not save badge_commits: {}", c);
        }
    }

    #[test]
    fn create_plastic_badge_with_label_msg_links() {
        let mut badge = Badge {
            label_text: String::from("version"),
            msg_text: String::from("1.2.3"),
            label_color: "#555".parse().unwrap(),
            msg_color: "#007ec6".parse().unwrap(),
            label_link: String::from("http://www.tangramvision.com"),
            msg_link: String::from("http://www.google.com"),
            ..Default::default()
        };

        let ci_path = std::env::current_dir().unwrap();

        let svg_path = ci_path.join(Path::new("plastic_badge_two_link.svg"));
        println!("Saving badge to {:#?}", svg_path);
        if let Err(c) = fs::write(svg_path, badge.generate_plastic_svg()) {
            println!("ERROR: Could not save badge_commits: {}", c);
        }
    }
}
