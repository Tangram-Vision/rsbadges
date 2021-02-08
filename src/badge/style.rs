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

use super::badge_type::*;
use super::generate_layout;
use super::generate_svg;

#[derive(Debug)]
pub enum Style {
    Plastic(Badge),
    Flat(Badge),
    FlatSquare(Badge),
    ForTheBadge(Badge),
}

impl Style {
    pub fn generate_svg(&self) -> Result<String, BadgeError> {
        let layout = match self {
            Style::Flat(badge) => generate_layout::flat_or_square(badge)?,
            Style::FlatSquare(badge) => generate_layout::flat_or_square(badge)?,
            Style::Plastic(badge) => generate_layout::plastic(badge)?,
            _ => Layout::default(),
        };

        let style = match self {
            Style::Flat(badge) => generate_svg::flat_svg(badge, layout)?,
            Style::FlatSquare(badge) => generate_svg::flat_square_svg(badge, layout)?,
            Style::Plastic(badge) => generate_svg::plastic_svg(badge, layout)?,
            _ => String::from("Not implemented yet!"),
        };

        Ok(style)
    }
}
