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

//! Generate an SVG from badge information

use super::badge_type::*;
use super::format_helper;
use askama::Template;
use rand::{distributions::Alphanumeric, Rng};

#[derive(Template, Debug)]
#[template(path = "badge_template_flat.xml", escape = "xml")]
/// Holds all information necessary for a Flat badge template.
struct BadgeTemplateFlat<'a> {
    /// text for label (left side)
    label_text: &'a str,
    /// text for message (right side)
    msg_text: &'a str,
    /// link for entire badge
    badge_link: &'a str,
    /// URL link for label
    label_link: &'a str,
    /// URL link for message
    msg_link: &'a str,
    /// color for label
    label_color: &'a str,
    /// color for message
    msg_color: &'a str,
    /// link to logo, placed on label side
    logo: &'a str,
    /// title of badge
    full_badge_title: &'a str,
    /// title for label
    label_title: &'a str,
    /// title for message
    msg_title: &'a str,
    /// height of the badge, in px
    badge_height: f32,
    /// logo width of the badge, in px
    logo_width: f32,
    /// logo placement in x
    logo_x: f32,
    /// logo placement in y
    logo_y: f32,
    /// width of text for label, in px
    label_text_width: f32,
    /// width of text for message, in px
    msg_text_width: f32,
    /// logo text placement in x
    label_text_x: f32,
    /// message text placement in x
    msg_text_x: f32,
    /// width of left side of badge
    left_width: f32,
    /// width of right side of badge
    right_width: f32,
    /// unique ID for smooth gradient
    id_smooth: &'a str,
    /// unique ID for round shape
    id_round: &'a str,
}

/// Minify SVG string
fn minify_svg_str(svg_str: String) -> String {
    svg_str
        .chars()
        .filter(|c| *c != '\r' && *c != '\n')
        .collect::<String>()
}

/// Generate the SVG string corresponding to a Flat badge with this Badge info
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
    Ok(minify_svg_str(flat_badge.render().unwrap()))
}

#[derive(Template, Debug)]
#[template(path = "badge_template_plastic.xml", escape = "xml")]
/// Holds all information necessary for a Plastic badge template.
struct BadgeTemplatePlastic<'a> {
    /// text for label (left side)
    label_text: &'a str,
    /// text for message (right side)
    msg_text: &'a str,
    /// link for entire badge
    badge_link: &'a str,
    /// URL link for label
    label_link: &'a str,
    /// URL link for message
    msg_link: &'a str,
    /// color for label
    label_color: &'a str,
    /// color for message
    msg_color: &'a str,
    /// link to logo, placed on label side
    logo: &'a str,
    /// title of badge
    full_badge_title: &'a str,
    /// title for label
    label_title: &'a str,
    /// title for message
    msg_title: &'a str,
    /// height of the badge, in px
    badge_height: f32,
    /// logo width of the badge, in px
    logo_width: f32,
    /// logo placement in x
    logo_x: f32,
    /// logo placement in y
    logo_y: f32,
    /// width of text for label, in px
    label_text_width: f32,
    /// width of text for message, in px
    msg_text_width: f32,
    /// logo text placement in x
    label_text_x: f32,
    /// message text placement in x
    msg_text_x: f32,
    /// width of left side of badge
    left_width: f32,
    /// width of right side of badge
    right_width: f32,
    /// unique ID for smooth gradient
    id_smooth: &'a str,
    /// unique ID for round shape
    id_round: &'a str,
}

/// Generate the SVG string corresponding to a Plastic badge with this Badge info
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
    Ok(minify_svg_str(plastic_badge.render().unwrap()))
}

#[derive(Template, Debug)]
#[template(path = "badge_template_flat_square.xml", escape = "xml")]
/// Holds all information necessary for a Flat Square badge template.
struct BadgeTemplateFlatSquare<'a> {
    /// text for label (left side)
    label_text: &'a str,
    /// text for message (right side)
    msg_text: &'a str,
    /// link for entire badge
    badge_link: &'a str,
    /// URL link for label
    label_link: &'a str,
    /// URL link for message
    msg_link: &'a str,
    /// color for label
    label_color: &'a str,
    /// color for message
    msg_color: &'a str,
    /// link to logo, placed on label side
    logo: &'a str,
    /// title of badge
    full_badge_title: &'a str,
    /// title for label
    label_title: &'a str,
    /// title for message
    msg_title: &'a str,
    /// height of the badge, in px
    badge_height: f32,
    /// logo width of the badge, in px
    logo_width: f32,
    /// logo placement in x
    logo_x: f32,
    /// logo placement in y
    logo_y: f32,
    /// width of text for label, in px
    label_text_width: f32,
    /// width of text for message, in px
    msg_text_width: f32,
    /// logo text placement in x
    label_text_x: f32,
    /// message text placement in x
    msg_text_x: f32,
    /// width of left side of badge
    left_width: f32,
    /// width of right side of badge
    right_width: f32,
}

/// Generate the SVG string corresponding to a Flat Square badge with this Badge info
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
        logo_x: layout.logo_x,
        logo_y: layout.logo_y,
        label_text_width: layout.label_text_width,
        msg_text_width: layout.msg_text_width,
        label_text_x: layout.label_text_x,
        msg_text_x: layout.msg_text_x,
        left_width: layout.label_total_width,
        right_width: layout.msg_total_width,
    };
    Ok(minify_svg_str(flat_square_badge.render().unwrap()))
}

#[derive(Template, Debug)]
#[template(path = "badge_template_forthebadge.xml", escape = "xml")]
/// Holds all information necessary for a "For the Badge" badge template.
struct BadgeTemplateForTheBadge<'a> {
    /// text for label (left side)
    label_text: &'a str,
    /// text for message (right side)
    msg_text: &'a str,
    /// link for entire badge
    badge_link: &'a str,
    /// URL link for label
    label_link: &'a str,
    /// URL link for message
    msg_link: &'a str,
    /// color for label
    label_color: &'a str,
    /// color for message
    msg_color: &'a str,
    /// link to logo, placed on label side
    logo: &'a str,
    /// title of badge
    full_badge_title: &'a str,
    /// title for label
    label_title: &'a str,
    /// title for message
    msg_title: &'a str,
    /// height of the badge, in px
    badge_height: f32,
    /// logo width of the badge, in px
    logo_width: f32,
    /// logo placement in x
    logo_x: f32,
    /// logo placement in y
    logo_y: f32,
    /// width of text for label, in px
    label_text_width: f32,
    /// width of text for message, in px
    msg_text_width: f32,
    /// logo text placement in x
    label_text_x: f32,
    /// message text placement in x
    msg_text_x: f32,
    /// width of left side of badge
    left_width: f32,
    /// width of right side of badge
    right_width: f32,
}

/// Generate the SVG string corresponding to a "for the badge" badge with this Badge info
pub(crate) fn for_the_badge_svg(badge: &Badge, layout: Layout) -> Result<String, BadgeError> {
    let mut logo_uri = badge.logo.clone();
    if badge.embed_logo {
        logo_uri = format_helper::attempt_logo_download(&badge.logo)?;
    }
    let forthebadge_badge = BadgeTemplateForTheBadge {
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
        logo_x: layout.logo_x,
        logo_y: layout.logo_y,
        label_text_width: layout.label_text_width,
        msg_text_width: layout.msg_text_width,
        label_text_x: layout.label_text_x,
        msg_text_x: layout.msg_text_x,
        left_width: layout.label_total_width,
        right_width: layout.msg_total_width,
    };
    Ok(minify_svg_str(forthebadge_badge.render().unwrap()))
}

#[derive(Template, Debug)]
#[template(path = "badge_template_social.xml", escape = "xml")]
/// Holds all information necessary for a Social badge template.
struct BadgeTemplateSocial<'a> {
    /// text for label (left side)
    label_text: &'a str,
    /// text for message (right side)
    msg_text: &'a str,
    /// link for entire badge
    badge_link: &'a str,
    /// URL link for label
    label_link: &'a str,
    /// URL link for message
    msg_link: &'a str,
    /// link to logo, placed on label side
    logo: &'a str,
    /// title of badge
    full_badge_title: &'a str,
    /// height of the badge, in px
    badge_height: f32,
    /// logo width of the badge, in px
    logo_width: f32,
    /// logo placement in x
    logo_x: f32,
    /// logo placement in y
    logo_y: f32,
    /// width of text for label, in px
    label_text_width: f32,
    /// width of text for message, in px
    msg_text_width: f32,
    /// logo text placement in x
    label_text_x: f32,
    /// message text placement in x
    msg_text_x: f32,
    /// width of left side of badge
    left_width: f32,
    /// width of right side of badge
    right_width: f32,
    /// unique ID for smooth gradient
    id_smooth: &'a str,
    /// unique ID for round shape
    id_round: &'a str,
    /// message bubble placement in x
    msg_bubble_x: f32,
    /// width of label bounding box
    label_rect_width: f32,
    /// width of message bounding box
    msg_rect_width: f32,
}

/// Generate the SVG string corresponding to a Social badge with this Badge info
pub(crate) fn social_svg(badge: &Badge, layout: Layout) -> Result<String, BadgeError> {
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
    let social_badge = BadgeTemplateSocial {
        label_text: &layout.label_text_norm,
        msg_text: &layout.msg_text_norm,
        badge_link: &badge.badge_link,
        label_link: &badge.label_link,
        msg_link: &badge.msg_link,
        logo: &logo_uri,
        full_badge_title: &badge.badge_title,
        badge_height: layout.badge_height,
        logo_width: layout.logo_width,
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
        msg_bubble_x: layout.msg_bubble_x,
        label_rect_width: layout.label_rect_width,
        msg_rect_width: layout.msg_rect_width,
    };
    Ok(minify_svg_str(social_badge.render().unwrap()))
}
