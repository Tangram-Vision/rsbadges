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

//! Used to set the generic info present on the badge.

use thiserror::Error;

/// Holds generic info about a badge.
///
/// The Badge struct holds all of the options that could be set for a badge SVG.
/// It is meant to be as superficial as possible; errors are checked on
/// the generation of the SVG itself, not the Badge object.
///
/// Since there are so many options, it's recommended that one use the `default()` method
/// during initialization to make the process less verbose:
///
/// ```
/// # use rsbadges::Badge;
/// let badge = Badge{
///     label_text: String::from("Custom_label"),
///     msg_text: String::from("Custom_msg"),
///     ..Badge::default()
/// };
/// ```
///
/// `label_color` and `msg_color` take any valid CSS color code. See
/// [the CSS color picker](https://www.w3schools.com/colors/colors_picker.asp)
/// for examples.
///
/// ```
/// let valid_rgb_color = "rgb(40, 20, 50)";
/// let valid_hsl_color = "hsl(15, 100%, 50%)";
/// let valid_hex_color = "#00bfff";
/// let valid_html_color = "white";
/// ```
///
#[derive(Debug, Clone)]
pub struct Badge {
    /// The text to show on the left side of the badge.
    pub label_text: String,
    /// The background color of the left side of the badge.
    pub label_color: String,
    /// The url to redirect to when the left side of the badge is clicked.
    pub label_link: String,
    /// The text to show on the right side of the badge.
    pub msg_text: String,
    /// The background color of the right side of the badge.
    pub msg_color: String,
    /// The url to redirect to when the right side of the badge is clicked.
    pub msg_link: String,
    /// A URI reference to a logo to display in the badge. Must be in SVG format.
    pub logo: String,
    /// Include the specified logo data directly in the badge.
    /// This prevents a URL call whenever the SVG is loaded.
    /// Only works if --logo is a HTTP/HTTPS URI or a valid file path.
    pub embed_logo: bool,
    /// The url to redirect to when any part of the badge is clicked.
    /// Overwrites --label-link and --msg-link.
    pub badge_link: String,
    /// The title to associate with the entire badge. More info
    /// [here](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/title).
    pub badge_title: String,
    /// The title to associate with the left side of the badge.
    pub label_title: String,
    /// The title to associate with the right side of the badge.
    pub msg_title: String,
}

impl Default for Badge {
    fn default() -> Badge {
        Badge {
            label_text: String::from("test"),
            msg_text: String::from("test"),
            badge_link: String::from(""),
            label_link: String::from(""),
            msg_link: String::from(""),
            label_color: String::from("#555"),
            msg_color: String::from("#007ec6"),
            logo: String::from(""),
            embed_logo: false,
            badge_title: String::from(""),
            label_title: String::from(""),
            msg_title: String::from(""),
        }
    }
}

#[derive(Default, Debug)]
pub(crate) struct Layout {
    pub label_text_norm: String,
    pub msg_text_norm: String,
    pub badge_height: f32,
    pub label_text_width: f32,
    pub msg_text_width: f32,
    pub label_total_width: f32,
    pub msg_total_width: f32,
    pub label_text_x: f32,
    pub msg_text_x: f32,
    pub logo_padding: f32,
    pub logo_width: f32,
    pub logo_x: f32,
    pub logo_y: f32,
    pub label_color: String,
    pub msg_color: String,
}

/// Error types that may occur on badge generation.
///
/// Badges are lazily evaluated, in a fashion; their data is not verified
/// until they are actually generated into SVGs. RSBadges tries to be loud
/// about malformed data or prevented actions via the error types below.
#[derive(Error, Debug, PartialEq)]
pub enum BadgeError {
    /// The arguments passed to the command line cannot be successfully parsed.
    #[error("Unable to parse command line arguments. {0}")]
    BadCommandLineArgs(String),
    /// A color assigned to the Badge does not have a valid CSS color format.
    /// See [the CSS color picker](https://www.w3schools.com/colors/colors_picker.asp)
    /// for vaild examples.
    #[error("The provided color {0} is not a valid CSS color format.")]
    ColorNotValid(String),
    /// RSBadges is unable to save the generated badge to an SVG. This is usually
    /// a file system error, not an error with badge generation.
    #[error("Unable to save the badge SVG to {0}.")]
    CannotSaveToFile(String),
    /// RSBadges is unable to download the logo specified, and therefore cannot embed the
    /// data into the SVG. This is usually due to a malformed logo URI.
    #[error("Unable to download and embed the logo. Attempted to load from {0}.")]
    CannotEmbedLogo(String),
    /// RSBadges can't find the font file it uses to measure out the width of the badge.
    #[error("Unable to find the font file.")]
    CannotLocateFont,
    /// RSBadges can't load the font file it uses to measure out the width of the badge.
    /// This probably means the file has somehow become corrupted.
    #[error("Unable to load the font file.")]
    CannotLoadFont,
    /// RSBadges has received a request to create a badge type it does not know about.
    /// This can only happen from the command line, since a Style is an enum via the API.
    #[error("{0} is an invalid style. Valid styles: \n- plastic\n- flat\n- flatsquare.")]
    InvalidStyle(String),
}
