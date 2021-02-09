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

use rsbadges::{Badge, BadgeError, Style};

#[test]
fn error_color_not_valid() {
    let badge = Badge {
        label_color: String::from("#t"),
        ..Badge::default()
    };
    match Style::Flat(badge.clone()).generate_svg() {
        Err(BadgeError::ColorNotValid(_)) => {}
        _ => unreachable!(),
    }
    match Style::FlatSquare(badge.clone()).generate_svg() {
        Err(BadgeError::ColorNotValid(_)) => {}
        _ => unreachable!(),
    }
    match Style::Plastic(badge.clone()).generate_svg() {
        Err(BadgeError::ColorNotValid(_)) => {}
        _ => unreachable!(),
    }
    match Style::ForTheBadge(badge).generate_svg() {
        Err(BadgeError::ColorNotValid(_)) => {}
        _ => unreachable!(),
    }

    let badge = Badge {
        msg_color: String::from("rgb(300.0)"),
        ..Badge::default()
    };
    match Style::Flat(badge.clone()).generate_svg() {
        Err(BadgeError::ColorNotValid(_)) => {}
        _ => unreachable!(),
    }
    match Style::FlatSquare(badge.clone()).generate_svg() {
        Err(BadgeError::ColorNotValid(_)) => {}
        _ => unreachable!(),
    }
    match Style::Plastic(badge.clone()).generate_svg() {
        Err(BadgeError::ColorNotValid(_)) => {}
        _ => unreachable!(),
    }
    match Style::ForTheBadge(badge).generate_svg() {
        Err(BadgeError::ColorNotValid(_)) => {}
        _ => unreachable!(),
    }
}

#[test]
fn error_cannot_embed_logo() {
    let badge = Badge {
        logo: String::from("bad_dir_nothing_here"),
        embed_logo: true,
        ..Badge::default()
    };
    // Flat and FlatSquare use the same code here, so no
    // need to test twice
    match Style::Flat(badge.clone()).generate_svg() {
        Err(BadgeError::CannotEmbedLogo(_)) => {}
        _ => unreachable!(),
    }
    match Style::Plastic(badge.clone()).generate_svg() {
        Err(BadgeError::CannotEmbedLogo(_)) => {}
        _ => unreachable!(),
    }
    match Style::ForTheBadge(badge).generate_svg() {
        Err(BadgeError::CannotEmbedLogo(_)) => {}
        _ => unreachable!(),
    }
}
