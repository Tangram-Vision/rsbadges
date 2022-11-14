// BSD 3-Clause License
//
// Copyright (c) 2021 RSBadges Authors
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
//    this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors
//    may be used to endorse or promote products derived from this software
//    without specific prior written permission.
//
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

//! Different helper functions used when formatting a badge for SVG generation.

use super::badge_type::BadgeError;
use css_color::Rgba;
use rusttype::{point, Font, Scale};
use std::path::Path;
use unicode_normalization::UnicodeNormalization;

/// Load a font into Rust.
/// Docs: https://gitlab.redox-os.org/redox-os/rusttype/-/blob/master/dev/examples/ascii.rs
pub fn load_font<'a>(bytes: &'static [u8]) -> Result<Font<'a>, BadgeError> {
    match Font::try_from_bytes(bytes) {
        Some(f) => Ok(f),
        None => Err(BadgeError::CannotLoadFont),
    }
}

/// Produce the text dimensions given the font, text, and requested size of the
/// string.
pub fn get_text_dims(font: &Font, text: &str, font_size: f32) -> (String, f32) {
    let norm_text = text.nfc().collect::<String>();
    let scale = Scale::uniform(font_size);
    let layout = font.layout(&norm_text, scale, point(0.0, 0.0));
    let mut glyphs_width = layout.fold(0.0, |acc, x| {
        acc + x.into_unpositioned().h_metrics().advance_width
    });
    if glyphs_width as usize % 2 == 0 {
        glyphs_width += 1.0;
    }
    (norm_text, glyphs_width)
}

/// Verify that the string passed in is a valid color.
pub fn verify_color(color: &str) -> Result<String, BadgeError> {
    let valid_color: Rgba = match color.parse() {
        Ok(c) => c,
        Err(_) => return Err(BadgeError::ColorNotValid(String::from(color))),
    };
    Ok(format!(
        "rgb({}, {}, {})",
        valid_color.red * 255.0,
        valid_color.green * 255.0,
        valid_color.blue * 255.0
    ))
}

// Thanks, Shepmaster.
// https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
/// Make the first character of the given string be uppercase.
pub fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Create an embeddable logo from the given URI.
pub fn create_embedded_logo(logo_uri: &str) -> Result<String, ureq::Error> {
    Ok(ureq::get(logo_uri).call()?.into_string()?)
}

/// Attempt to download a logo from a given URI. This can be a web URL or a local path.
pub fn attempt_logo_download(logo_uri: &str) -> Result<String, BadgeError> {
    // Check for local copy
    let local_path = Path::new(logo_uri);

    let data = match std::fs::read_to_string(local_path) {
        Ok(f) => f,
        Err(_) => match create_embedded_logo(logo_uri) {
            Ok(logo_data) => logo_data,
            Err(_) => return Err(BadgeError::CannotEmbedLogo(String::from(logo_uri))),
        },
    };

    // If not local, download
    Ok(format!(
        "data:image/svg+xml;base64,{}",
        base64::encode(data.as_bytes())
    ))
}
