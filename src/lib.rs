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
//! RSBadges is a Rust-friendly badge generator. The interface strives to be minimal
//! while still providing a feature-rich API. Both the label (the left side) and the
//! message (the right side) of the badge can be customized fully, with the ability to
//!
//! - Set text
//! - Set color using any valid CSS color code
//! - Embed a link into each side or a link for the whole badge
//! - Add a logo from a local source or a URL
//! - Embed that logo's data into the SVG directly
//! - Set the style of badge, as described in [Shields.io](http://shields.io)
//!
//! RSBadges can be used as an API or a command line interface (CLI).
//!
//! # API
//!
//! First, instantiate a [Badge] struct to set all of the generic options for a badge SVG.
//! This fully-populated Badge is then wrapped in a [Style] enum, which indicates which
//! style of badge to eventually generate.
//!
//! ```
//! use rsbadges::{Badge, Style};
//! let badge = Badge{
//!     label_text: String::from("Custom_label"),
//!     msg_text: String::from("Custom_msg"),
//!     label_color: String::from("red"),
//!     ..Badge::default()
//! };
//! // Create a plastic badge using the data created above.
//! let badge_style = Style::Plastic(badge);
//! ```
//!
//! Badge and Style together are sufficient to
//! tell RSBadges how to construct the right badge, which it does through `generate_svg()`:
//!
//! ```
//! # use rsbadges::{Badge, Style};
//! # let badge = Badge{
//! #     label_text: String::from("Custom_label"),
//! #     msg_text: String::from("Custom_msg"),
//! #     label_color: String::from("red"),
//! #     ..Badge::default()
//! # };
//! # // Create a plastic badge using the data created above.
//! # let badge_style = Style::Plastic(badge);
//! let badge_svg = badge_style.generate_svg().unwrap();
//! ```
//!
//! The resulting SVG String can be saved to file using the convenience function `save_svg()`:
//!
//! ```
//! # use rsbadges::{Badge, Style};
//! # let badge = Badge{
//! #     label_text: String::from("Custom_label"),
//! #     msg_text: String::from("Custom_msg"),
//! #     label_color: String::from("red"),
//! #     ..Badge::default()
//! # };
//! # // Create a plastic badge using the data created above.
//! # let badge_style = Style::Plastic(badge);
//! # let badge_svg = badge_style.generate_svg().unwrap();
//! rsbadges::save_svg("~/Downloads/badge.svg", &badge_svg);
//! ```
//!
//! See the [Badge] and [Style] documentation for more.
//!
//! # CLI
//!
//! The CLI features all of the customization options from the API, along with a
//! few quality-of-life improvements for command line use and evaluation, such as
//!
//! - Opening a created badge SVG in browser after creation
//! - Specifying a save directory for the SVG
//!
//! Just like the API, `label-color` and `msg-color` take any valid CSS color
//! as input. Don't worry if you get it wrong; RSBadges will let you know.
//!
//! | Short      | Long                                   | Default
//! | ---------  | ------------------------------------   | -------
//! | `-a`       | `--label <string>`                     | "test"
//! | `-b`       | `--label-color <css_color>`            | "#555"
//! | `-c`       | `--label-link <url>`                   | ""
//! | `-x`       | `--msg <string>`                       | "test"
//! | `-y`       | `--msg-color <css_color>`              | "#007ec6"
//! | `-z`       | `--msg-link <url>`                     | ""
//! | `-l`       | `--logo <url or local path>`           | ""
//! | `-f`       | `--save-to-svg-at <filepath/file.svg>` | ""
//! | `-s`       | `--style <plastic,flat,flatsquare>`    | "flat"
//! | `-o`       | `--open-in-browser`                    | false
//! | `-h`       | `--help`                               | false
//! | `-e`       | `--embed-logo`                         | false
//!
//! Run the CLI with the `-h` flag to see all possible arguments and flags.
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
