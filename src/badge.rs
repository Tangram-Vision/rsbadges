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
use unicode_normalization::UnicodeNormalization;

// Keep these grouped...
#[derive(Template, Debug)]
#[template(path = "badge_template_flat.xml", escape = "xml")]
struct BadgeTemplateFlat<'a> {
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
    badge_height: usize,
    logo_width: usize,
    logo_padding: usize,
    logo_x: usize,
    logo_y: usize,
    label_text_width: usize,
    msg_text_width: usize,
    label_text_x: usize,
    msg_text_x: usize,
    left_width: usize,
    right_width: usize,
    id_smooth: &'a str,
    id_round: &'a str,
}

#[derive(Template, Debug)]
#[template(path = "badge_template_plastic.xml", escape = "xml")]
struct BadgeTemplatePlastic<'a> {
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
    badge_height: usize,
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

#[derive(Template, Debug)]
#[template(path = "badge_template_flat_square.xml", escape = "xml")]
struct BadgeTemplateFlatSquare<'a> {
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
    badge_height: usize,
    logo_width: usize,
    logo_padding: usize,
    label_text_width: usize,
    msg_text_width: usize,
    label_text_x: usize,
    msg_text_x: usize,
    left_width: usize,
    right_width: usize,
}

// Docs: https://gitlab.redox-os.org/redox-os/rusttype/-/blob/master/dev/examples/ascii.rs
fn load_regular_font<'a>() -> Font<'a> {
    let font_data = include_bytes!("../fonts/DejaVuSans.ttf");
    Font::try_from_bytes(font_data as &[u8]).expect("error constructing a Font from bytes")
}

fn load_bold_font<'a>() -> Font<'a> {
    let font_data = include_bytes!("../fonts/DejaVuSans-Bold.ttf");
    Font::try_from_bytes(font_data as &[u8]).expect("error constructing a Font from bytes")
}

fn get_text_dims(font: &Font, text: &str, font_size: f32, kerning_pix: f32) -> (f32, f32) {
    let norm_text = text.nfc().collect::<String>();
    let scale = Scale::uniform(font_size);
    let layout = font.layout(&norm_text, scale, point(0.0, 0.0));
    let mut glyphs_width = layout.fold(0.0, |acc, x| {
        acc + x.into_unpositioned().h_metrics().advance_width
    });
    let v_metrics = font.v_metrics(scale);
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil();
    // 1px padding
    glyphs_width += norm_text.len() as f32 * kerning_pix;
    if glyphs_width as usize % 2 == 0 {
        glyphs_width += 1.0;
    }
    (glyphs_width, glyphs_height)
}

#[derive(Debug)]
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
}

impl Default for Badge {
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

#[derive(Debug, PartialEq)]
enum Flavor {
    Plastic,
    Flat,
    FlatSquare,
    ForTheBadge,
}

#[derive(Default, Debug)]
pub struct DerivedInfo {
    badge_height: f32,
    label_text_width: f32,
    msg_text_width: f32,
    label_total_width: f32,
    msg_total_width: f32,
    label_text_x: f32,
    msg_text_x: f32,
    logo_padding: f32,
    logo_width: f32,
    logo_x: f32,
    logo_y: f32,
    label_color: String,
    msg_color: String,
}

impl Badge {
    fn derive_construction_info(&mut self, flavor: Flavor) -> DerivedInfo {
        let mut derived_info = DerivedInfo::default();

        // Normalize text
        let font = load_regular_font();
        let bold_font = load_bold_font();
        if flavor == Flavor::ForTheBadge {
            self.label_text = self.label_text.to_uppercase();
            self.msg_text = self.msg_text.to_uppercase();
            derived_info.label_text_width = get_text_dims(&font, &self.label_text, 10.0, 0.8).0;
            derived_info.msg_text_width = get_text_dims(&bold_font, &self.msg_text, 10.0, 0.8).0;
        } else {
            derived_info.label_text_width = get_text_dims(&font, &self.label_text, 11.0, 0.8).0;
            derived_info.msg_text_width = get_text_dims(&font, &self.msg_text, 11.0, 0.8).0;
        }

        // Padding and spacing calculations
        let horiz_padding = match flavor {
            Flavor::ForTheBadge => 9.0,
            _ => 5.0,
        };
        derived_info.badge_height = match flavor {
            Flavor::ForTheBadge => 28.0,
            Flavor::Plastic => 18.0,
            Flavor::Flat => 20.0,
            Flavor::FlatSquare => 20.0,
            // Flavor::Social => 20.0,
        };

        // Logo padding and width
        let mut total_logo_width = 0.0;
        derived_info.logo_width = 0.0;
        derived_info.logo_padding = 0.0;
        if !self.logo.is_empty() {
            if !self.label_text.is_empty() {
                derived_info.logo_padding = 3.0;
            }
            let logo_height = 14.0; // hardcoded
            derived_info.logo_y = (derived_info.badge_height - logo_height) * 0.5;
            derived_info.logo_x = horiz_padding;
            derived_info.logo_width = 14.0;
            total_logo_width = derived_info.logo_width + derived_info.logo_padding;
        }

        if flavor == Flavor::ForTheBadge {
            // TODO: Complete for ForTheBadge
            // // Label padding and width
            // derived_info.label_total_width = derived_info.label_text_width + 10.0;
            // if !self.label_text.is_empty() {
            //     derived_info.label_total_width += 10.0 + (2.0 * self.label_text.len() as f32);
            // } else if !self.logo.is_empty() {
            //     if !self.label_text.is_empty() {
            //         derived_info.label_total_width += 7.0;
            //     } else {
            //         derived_info.label_total_width -= 7.0;
            //     }
            // } else {
            //     derived_info.label_total_width -= 11.0;
            // }

            // derived_info.label_text_x = (derived_info.label_total_width + total_logo_width) / 2.0;
            // derived_info.label_text_width =
            //     derived_info.label_total_width - (24.0 + total_logo_width);

            // // Message padding and width
            // derived_info.msg_total_width = derived_info.msg_text_width * 1.5 + 20.0;
            // let left_width = 0;
            // let

            // let mut msg_text_margin = derived_info.label_total_width;
            // if !self.msg_text.is_empty() {
            //     msg_text_margin -= 1.0;
            // }

            // if self.label_text.is_empty() {
            //     if !self.logo.is_empty() {
            //         msg_text_margin += derived_info.logo_width + horiz_padding;
            //     } else {
            //         msg_text_margin += 1.0;
            //     }
            // }

            // derived_info.msg_total_width = derived_info.msg_text_width + (2.0 * horiz_padding);
            // if !self.logo.is_empty() && self.label_text.is_empty() {
            //     derived_info.msg_total_width += derived_info.logo_width + horiz_padding - 1.0;
            // }

            // derived_info.msg_text_x =
            //     msg_text_margin + (0.5 * derived_info.msg_text_width) + horiz_padding;
        } else {
            // Label padding and width
            let label_text_margin = total_logo_width + 1.0;
            derived_info.label_total_width = 0.0;
            if !self.label_text.is_empty() {
                derived_info.label_total_width =
                    derived_info.label_text_width + (2.0 * horiz_padding) + total_logo_width;
            }
            derived_info.label_text_x =
                label_text_margin + (0.5 * derived_info.label_text_width) + horiz_padding;

            // Message padding and width
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

            derived_info.msg_text_x =
                msg_text_margin + (0.5 * derived_info.msg_text_width) + horiz_padding;
        }

        // Scale back up for the SVG
        derived_info.label_text_width *= 10.0;
        derived_info.msg_text_width *= 10.0;
        derived_info.label_text_x *= 10.0;
        derived_info.msg_text_x *= 10.0;

        // Color conversion to string
        derived_info.label_color = color_to_string(self.label_color);
        derived_info.msg_color = color_to_string(self.msg_color);

        derived_info
    }

    pub fn generate_flat_svg(&mut self) -> String {
        let derived_info = self.derive_construction_info(Flavor::Flat);
        let id_suffix: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let id_smooth = format!("smooth{}", id_suffix);
        let id_round = format!("round{}", id_suffix);
        let flat_badge = BadgeTemplateFlat {
            label_text: &self.label_text,
            msg_text: &self.msg_text,
            badge_link: &self.badge_link,
            label_link: &self.label_link,
            msg_link: &self.msg_link,
            label_color: &derived_info.label_color,
            msg_color: &derived_info.msg_color,
            logo: &self.logo,
            full_badge_title: &self.badge_title,
            label_title: &self.label_title,
            msg_title: &self.msg_title,
            badge_height: derived_info.badge_height as usize,
            logo_width: derived_info.logo_width as usize,
            logo_padding: derived_info.logo_padding as usize,
            logo_x: derived_info.logo_x as usize,
            logo_y: derived_info.logo_y as usize,
            label_text_width: derived_info.label_text_width as usize,
            msg_text_width: derived_info.msg_text_width as usize,
            label_text_x: derived_info.label_text_x as usize,
            msg_text_x: derived_info.msg_text_x as usize,
            left_width: derived_info.label_total_width as usize,
            right_width: derived_info.msg_total_width as usize,
            id_smooth: &id_smooth,
            id_round: &id_round,
        };

        flat_badge.render().unwrap()
    }

    pub fn generate_plastic_svg(&mut self) -> String {
        let derived_info = self.derive_construction_info(Flavor::Plastic);
        let id_suffix: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let id_smooth = format!("smooth{}", id_suffix);
        let id_round = format!("round{}", id_suffix);
        let plastic_badge = BadgeTemplatePlastic {
            label_text: &self.label_text,
            msg_text: &self.msg_text,
            badge_link: &self.badge_link,
            label_link: &self.label_link,
            msg_link: &self.msg_link,
            label_color: &derived_info.label_color,
            msg_color: &derived_info.msg_color,
            logo: &self.logo,
            full_badge_title: &self.badge_title,
            label_title: &self.label_title,
            msg_title: &self.msg_title,
            badge_height: derived_info.badge_height as usize,
            logo_width: derived_info.logo_width as usize,
            logo_padding: derived_info.logo_padding as usize,
            label_text_width: derived_info.label_text_width as usize,
            msg_text_width: derived_info.msg_text_width as usize,
            label_text_x: derived_info.label_text_x as usize,
            msg_text_x: derived_info.msg_text_x as usize,
            left_width: derived_info.label_total_width as usize,
            right_width: derived_info.msg_total_width as usize,
            id_smooth: &id_smooth,
            id_round: &id_round,
        };

        plastic_badge.render().unwrap()
    }

    pub fn generate_flat_square_svg(&mut self) -> String {
        let derived_info = self.derive_construction_info(Flavor::FlatSquare);
        let flat_square_badge = BadgeTemplateFlatSquare {
            label_text: &self.label_text,
            msg_text: &self.msg_text,
            badge_link: &self.badge_link,
            label_link: &self.label_link,
            msg_link: &self.msg_link,
            label_color: &derived_info.label_color,
            msg_color: &derived_info.msg_color,
            logo: &self.logo,
            full_badge_title: &self.badge_title,
            label_title: &self.label_title,
            msg_title: &self.msg_title,
            badge_height: derived_info.badge_height as usize,
            logo_width: derived_info.logo_width as usize,
            logo_padding: derived_info.logo_padding as usize,
            label_text_width: derived_info.label_text_width as usize,
            msg_text_width: derived_info.msg_text_width as usize,
            label_text_x: derived_info.label_text_x as usize,
            msg_text_x: derived_info.msg_text_x as usize,
            left_width: derived_info.label_total_width as usize,
            right_width: derived_info.msg_total_width as usize,
        };

        flat_square_badge.render().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use crate::badge::{Badge, Flavor};
    use std::fs;
    use std::path::Path;

    fn save_badge_to_tmp(filename: &str, badge: &mut Badge, flavor: Flavor) {
        let ci_path = std::env::temp_dir();
        // Flat badge
        let svg_path = ci_path.join(Path::new(filename));
        let svg = match flavor {
            Flavor::Flat => badge.generate_flat_svg(),
            Flavor::FlatSquare => badge.generate_flat_square_svg(),
            Flavor::Plastic => badge.generate_plastic_svg(),
            _ => "".to_string(),
        };
        println!("Saving badge to {:#?}", svg_path);
        if let Err(c) = fs::write(svg_path, svg) {
            println!("ERROR: Could not save badge: {}", c);
        }
    }

    #[test]
    fn create_badges() {
        let mut badge = Badge {
            label_text: String::from("version"),
            msg_text: String::from("1.2.3"),
            label_color: "#555".parse().unwrap(),
            msg_color: "#007ec6".parse().unwrap(),
            ..Default::default()
        };

        save_badge_to_tmp("flat_badge.svg", &mut badge, Flavor::Flat);
        save_badge_to_tmp("flat_square_badge.svg", &mut badge, Flavor::FlatSquare);
        save_badge_to_tmp("plastic_badge.svg", &mut badge, Flavor::Plastic);
    }

    #[test]
    fn create_badges_with_logos() {
        let mut badge = Badge {
            label_text: String::from("version"),
            msg_text: String::from("1.2.3"),
            label_color: "#555".parse().unwrap(),
            msg_color: "#007ec6".parse().unwrap(),
            logo: String::from("https://simpleicons.org/icons/slack.svg"),
            ..Default::default()
        };

        save_badge_to_tmp("flat_logo.svg", &mut badge, Flavor::Flat);
        save_badge_to_tmp("flat_square_logo.svg", &mut badge, Flavor::FlatSquare);
        save_badge_to_tmp("plastic_logo.svg", &mut badge, Flavor::Plastic);
    }

    #[test]
    fn create_badge_chinese_characters() {
        let mut badge = Badge {
            label_text: String::from("版"),
            msg_text: String::from("不知道"),
            label_color: "#555".parse().unwrap(),
            msg_color: "#007ec6".parse().unwrap(),
            ..Default::default()
        };

        save_badge_to_tmp("flat_chinese.svg", &mut badge, Flavor::Flat);
        save_badge_to_tmp("flat_square_chinese.svg", &mut badge, Flavor::FlatSquare);
        save_badge_to_tmp("plastic_chinese.svg", &mut badge, Flavor::Plastic);
    }

    #[test]
    fn create_badge_arabic_characters() {
        let mut badge = Badge {
            label_text: String::from("اختبار"),
            msg_text: String::from("انا لا اعرف"),
            label_color: "#555".parse().unwrap(),
            msg_color: "#007ec6".parse().unwrap(),
            ..Default::default()
        };

        save_badge_to_tmp("flat_arabic.svg", &mut badge, Flavor::Flat);
        save_badge_to_tmp("flat_square_arabic.svg", &mut badge, Flavor::FlatSquare);
        save_badge_to_tmp("plastic_arabic.svg", &mut badge, Flavor::Plastic);
    }

    #[test]
    fn create_badge_metal() {
        let mut badge = Badge {
            label_text: String::from("röck döts"),
            msg_text: String::from("metal"),
            label_color: "#555".parse().unwrap(),
            msg_color: "#007ec6".parse().unwrap(),
            ..Default::default()
        };

        save_badge_to_tmp("flat_metal.svg", &mut badge, Flavor::Flat);
        save_badge_to_tmp("flat_square_metal.svg", &mut badge, Flavor::FlatSquare);
        save_badge_to_tmp("plastic_metal.svg", &mut badge, Flavor::Plastic);
    }

    #[test]
    fn create_badges_badge_link() {
        let mut badge = Badge {
            label_text: String::from("version"),
            msg_text: String::from("1.2.3"),
            label_color: "#555".parse().unwrap(),
            msg_color: "#007ec6".parse().unwrap(),
            badge_link: String::from("http://www.tangramvision.com"),
            ..Default::default()
        };

        save_badge_to_tmp("flat_link.svg", &mut badge, Flavor::Flat);
        save_badge_to_tmp("flat_square_link.svg", &mut badge, Flavor::FlatSquare);
        save_badge_to_tmp("plastic_link.svg", &mut badge, Flavor::Plastic);
    }

    #[test]
    fn create_badges_with_label_msg_links() {
        let mut badge = Badge {
            label_text: String::from("version"),
            msg_text: String::from("1.2.3"),
            label_color: "#555".parse().unwrap(),
            msg_color: "#007ec6".parse().unwrap(),
            label_link: String::from("http://www.tangramvision.com"),
            msg_link: String::from("http://www.google.com"),
            ..Default::default()
        };

        save_badge_to_tmp("flat_links.svg", &mut badge, Flavor::Flat);
        save_badge_to_tmp("flat_square_links.svg", &mut badge, Flavor::FlatSquare);
        save_badge_to_tmp("plastic_links.svg", &mut badge, Flavor::Plastic);
    }
}
