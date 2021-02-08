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

//! Create shields.io-like badges from the comfort and safety of Rust
//!
//! RSBadges is a Rust-friendly badges generator. It acts as a library and a command line
//! interface. Most everything on a badge can be customized:
//!
//! - Badge text, for either side
//! - Badge links, for either side
//! - Badge color, for either side
//! - Added logo with the option to embed if desired
//!
//!  # Example usage
//!
//! ```
//! use rsbadges::{Badge, Style};
//! let badge = Badge{
//!     label_text: String::from("Custom_label"),
//!     msg_text: String::from("Custom_msg"),
//!     ..Badge::default()
//! };
//! // Create a plastic badge using the data created above.
//! let badge_style = Style::Plastic(badge);
//! let badge_svg = badge_style.generate_svg();
//! rsbadges::save_svg("~/Downloads/badge.svg", badge_svg);
//! ```
//!

#![warn(missing_docs)] // warn if there are missing docs

mod badge;

pub use badge::{Badge, BadgeError, Style};
use std::fs;
use std::path::Path;

/// A convenience function to save an SVG to a file.
///
/// # Errors
///
/// This will return [BadgeError::CannotSaveToFile] if the directory
/// is malformed or cannot be accessed.
pub fn save_svg(filepath: &str, svg: &str) -> Result<(), BadgeError> {
    let svg_path = Path::new(filepath);
    if let Err(c) = fs::write(svg_path, svg) {
        println!("Error: {}", c);
        return Err(BadgeError::CannotSaveToFile(String::from(filepath)));
    }
    Ok(())
}
