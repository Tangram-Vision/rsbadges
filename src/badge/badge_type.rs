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

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Badge {
    pub label_text: String,
    pub msg_text: String,
    pub badge_link: String,
    pub label_link: String,
    pub msg_link: String,
    pub label_color: String,
    pub msg_color: String,
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
            label_color: String::from("#555"),
            msg_color: String::from("#007ec6"),
            logo: String::from(""),
            embed_logo: false,
            badge_title: String::from(""),
            label_title: String::from(""),
            msg_title: String::from(""),
            open_in_browser: false,
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

#[derive(Error, Debug, PartialEq)]
pub enum BadgeError {
    #[error("Unable to parse command line arguments. {0}")]
    BadCommandLineArgs(String),
    #[error("The provided color {0} is not a valid CSS color format.")]
    ColorNotValid(String),
    #[error("Unable to save the badge SVG to {0}.")]
    CannotSaveToFile(String),
    #[error("Unable to download and embed the logo. Attempted to load from {0}.")]
    CannotEmbedLogo(String),
    #[error("Unable to find the font file.")]
    CannotLocateFont,
    #[error("Unable to load the font file.")]
    CannotLoadFont,
    #[error("{0} is an invalid style. Valid styles: \n- plastic\n- flat\n- flatsquare.")]
    InvalidStyle(String),
}
