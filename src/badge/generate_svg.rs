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
use super::format_helper;
use askama::Template;
use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;

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
    badge_height: f32,
    logo_width: f32,
    logo_padding: f32,
    logo_x: f32,
    logo_y: f32,
    label_text_width: f32,
    msg_text_width: f32,
    label_text_x: f32,
    msg_text_x: f32,
    left_width: f32,
    right_width: f32,
    id_smooth: &'a str,
    id_round: &'a str,
}

pub(crate) fn flat_svg(badge: &Badge, layout: Layout) -> Result<String, BadgeError> {
    let id_suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    let id_smooth = format!("smooth{}", id_suffix);
    let id_round = format!("round{}", id_suffix);
    let mut logo_uri = badge.logo.clone();
    if badge.embed_logo {
        logo_uri = format_helper::attempt_logo_download(&badge.logo)?;
    }
    let flat_badge = BadgeTemplateFlat {
        label_text: &layout.label_text_norm,
        msg_text: &layout.msg_text_norm,
        badge_link: &badge.badge_link,
        label_link: &badge.label_link,
        msg_link: &badge.msg_link,
        label_color: &layout.label_color,
        msg_color: &layout.msg_color,
        logo: &logo_uri,
        full_badge_title: &badge.badge_title,
        label_title: &badge.label_title,
        msg_title: &badge.msg_title,
        badge_height: layout.badge_height,
        logo_width: layout.logo_width,
        logo_padding: layout.logo_padding,
        logo_x: layout.logo_x,
        logo_y: layout.logo_y,
        label_text_width: layout.label_text_width,
        msg_text_width: layout.msg_text_width,
        label_text_x: layout.label_text_x,
        msg_text_x: layout.msg_text_x,
        left_width: layout.label_total_width,
        right_width: layout.msg_total_width,
        id_smooth: &id_smooth,
        id_round: &id_round,
    };
    let re = Regex::new("[\r\n]*").unwrap();
    Ok(String::from(
        re.replace_all(&flat_badge.render().unwrap(), ""),
    ))
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
    badge_height: f32,
    logo_width: f32,
    logo_padding: f32,
    logo_x: f32,
    logo_y: f32,
    label_text_width: f32,
    msg_text_width: f32,
    label_text_x: f32,
    msg_text_x: f32,
    left_width: f32,
    right_width: f32,
    id_smooth: &'a str,
    id_round: &'a str,
}

pub(crate) fn plastic_svg(badge: &Badge, layout: Layout) -> Result<String, BadgeError> {
    let id_suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    let id_smooth = format!("smooth{}", id_suffix);
    let id_round = format!("round{}", id_suffix);
    let mut logo_uri = badge.logo.clone();
    if badge.embed_logo {
        logo_uri = format_helper::attempt_logo_download(&badge.logo)?;
    }
    let plastic_badge = BadgeTemplatePlastic {
        label_text: &layout.label_text_norm,
        msg_text: &layout.msg_text_norm,
        badge_link: &badge.badge_link,
        label_link: &badge.label_link,
        msg_link: &badge.msg_link,
        label_color: &layout.label_color,
        msg_color: &layout.msg_color,
        logo: &logo_uri,
        full_badge_title: &badge.badge_title,
        label_title: &badge.label_title,
        msg_title: &badge.msg_title,
        badge_height: layout.badge_height,
        logo_width: layout.logo_width,
        logo_padding: layout.logo_padding,
        logo_x: layout.logo_x,
        logo_y: layout.logo_y,
        label_text_width: layout.label_text_width,
        msg_text_width: layout.msg_text_width,
        label_text_x: layout.label_text_x,
        msg_text_x: layout.msg_text_x,
        left_width: layout.label_total_width,
        right_width: layout.msg_total_width,
        id_smooth: &id_smooth,
        id_round: &id_round,
    };

    let re = Regex::new("[\r\n]*").unwrap();
    Ok(String::from(
        re.replace_all(&plastic_badge.render().unwrap(), ""),
    ))
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
    badge_height: f32,
    logo_width: f32,
    logo_padding: f32,
    logo_x: f32,
    logo_y: f32,
    label_text_width: f32,
    msg_text_width: f32,
    label_text_x: f32,
    msg_text_x: f32,
    left_width: f32,
    right_width: f32,
}

pub(crate) fn flat_square_svg(badge: &Badge, layout: Layout) -> Result<String, BadgeError> {
    let mut logo_uri = badge.logo.clone();
    if badge.embed_logo {
        logo_uri = format_helper::attempt_logo_download(&badge.logo)?;
    }
    let flat_square_badge = BadgeTemplateFlatSquare {
        label_text: &layout.label_text_norm,
        msg_text: &layout.msg_text_norm,
        badge_link: &badge.badge_link,
        label_link: &badge.label_link,
        msg_link: &badge.msg_link,
        label_color: &layout.label_color,
        msg_color: &layout.msg_color,
        logo: &logo_uri,
        full_badge_title: &badge.badge_title,
        label_title: &badge.label_title,
        msg_title: &badge.msg_title,
        badge_height: layout.badge_height,
        logo_width: layout.logo_width,
        logo_padding: layout.logo_padding,
        logo_x: layout.logo_x,
        logo_y: layout.logo_y,
        label_text_width: layout.label_text_width,
        msg_text_width: layout.msg_text_width,
        label_text_x: layout.label_text_x,
        msg_text_x: layout.msg_text_x,
        left_width: layout.label_total_width,
        right_width: layout.msg_total_width,
    };

    let re = Regex::new("[\r\n]*").unwrap();
    Ok(String::from(
        re.replace_all(&flat_square_badge.render().unwrap(), ""),
    ))
}
