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
use super::format_helper::*;

pub(crate) fn plastic(badge: &Badge) -> Result<Layout, BadgeError> {
    let mut layout = Layout::default();

    // Normalize text
    let font = load_font("fonts/verdana.ttf")?;
    let (label_text_norm, label_text_width) = get_text_dims(&font, &badge.label_text, 11.0, 0.8);
    layout.label_text_norm = label_text_norm;
    layout.label_text_width = label_text_width;
    let (msg_text_norm, msg_text_width) = get_text_dims(&font, &badge.msg_text, 11.0, 0.8);
    layout.msg_text_norm = msg_text_norm;
    layout.msg_text_width = msg_text_width;

    // Padding and spacing calculations
    let horiz_padding = 5.0;
    layout.badge_height = 18.0;

    // Logo padding and width
    let mut total_logo_width = 0.0;
    layout.logo_width = 0.0;
    layout.logo_padding = 0.0;
    if !badge.logo.is_empty() {
        if !badge.label_text.is_empty() {
            layout.logo_padding = 3.0;
        }
        let logo_height = 14.0;
        layout.logo_y = (layout.badge_height - logo_height) * 0.5;
        layout.logo_x = horiz_padding;
        layout.logo_width = 14.0;
        total_logo_width = layout.logo_width + layout.logo_padding;
    }

    // Label padding and width
    let label_text_margin = total_logo_width + 1.0;
    layout.label_total_width = 0.0;
    if !badge.label_text.is_empty() {
        layout.label_total_width =
            layout.label_text_width + (2.0 * horiz_padding) + total_logo_width;
    }
    layout.label_text_x = label_text_margin + (0.5 * layout.label_text_width) + horiz_padding;

    // Message padding and width
    let mut msg_text_margin = layout.label_total_width;
    if !badge.msg_text.is_empty() {
        msg_text_margin -= 1.0;
    }

    if badge.label_text.is_empty() {
        if !badge.logo.is_empty() {
            msg_text_margin += layout.logo_width + horiz_padding;
        } else {
            msg_text_margin += 1.0;
        }
    }

    layout.msg_total_width = layout.msg_text_width + (2.0 * horiz_padding);
    if !badge.logo.is_empty() && badge.label_text.is_empty() {
        layout.msg_total_width += layout.logo_width + horiz_padding - 1.0;
    }

    layout.msg_text_x = msg_text_margin + (0.5 * layout.msg_text_width) + horiz_padding;

    // Scale back up for the SVG
    layout.label_text_width *= 10.0;
    layout.msg_text_width *= 10.0;
    layout.label_text_x *= 10.0;
    layout.msg_text_x *= 10.0;

    // Color conversion to string
    layout.label_color = verify_color(&badge.label_color)?;
    layout.msg_color = verify_color(&badge.msg_color)?;

    Ok(layout)
}

pub(crate) fn flat_or_square(badge: &Badge) -> Result<Layout, BadgeError> {
    let mut layout = Layout::default();

    // Normalize text
    let font = load_font("fonts/verdana.ttf")?;
    let (label_text_norm, label_text_width) = get_text_dims(&font, &badge.label_text, 11.0, 0.8);
    layout.label_text_norm = label_text_norm;
    layout.label_text_width = label_text_width;
    let (msg_text_norm, msg_text_width) = get_text_dims(&font, &badge.msg_text, 11.0, 0.8);
    layout.msg_text_norm = msg_text_norm;
    layout.msg_text_width = msg_text_width;

    // Padding and spacing calculations
    let horiz_padding = 5.0;
    layout.badge_height = 20.0;

    // Logo padding and width
    let mut total_logo_width = 0.0;
    layout.logo_width = 0.0;
    layout.logo_padding = 0.0;
    if !badge.logo.is_empty() {
        if !badge.label_text.is_empty() {
            layout.logo_padding = 3.0;
        }
        let logo_height = 14.0;
        layout.logo_y = (layout.badge_height - logo_height) * 0.5;
        layout.logo_x = horiz_padding;
        layout.logo_width = 14.0;
        total_logo_width = layout.logo_width + layout.logo_padding;
    }

    // Label padding and width
    let label_text_margin = total_logo_width + 1.0;
    layout.label_total_width = 0.0;
    if !badge.label_text.is_empty() {
        layout.label_total_width =
            layout.label_text_width + (2.0 * horiz_padding) + total_logo_width;
    }
    layout.label_text_x = label_text_margin + (0.5 * layout.label_text_width) + horiz_padding;

    // Message padding and width
    let mut msg_text_margin = layout.label_total_width;
    if !badge.msg_text.is_empty() {
        msg_text_margin -= 1.0;
    }

    if badge.label_text.is_empty() {
        if !badge.logo.is_empty() {
            msg_text_margin += layout.logo_width + horiz_padding;
        } else {
            msg_text_margin += 1.0;
        }
    }

    layout.msg_total_width = layout.msg_text_width + (2.0 * horiz_padding);
    if !badge.logo.is_empty() && badge.label_text.is_empty() {
        layout.msg_total_width += layout.logo_width + horiz_padding - 1.0;
    }

    layout.msg_text_x = msg_text_margin + (0.5 * layout.msg_text_width) + horiz_padding;

    // Scale back up for the SVG
    layout.label_text_width *= 10.0;
    layout.msg_text_width *= 10.0;
    layout.label_text_x *= 10.0;
    layout.msg_text_x *= 10.0;

    // Color conversion to string
    layout.label_color = verify_color(&badge.label_color)?;
    layout.msg_color = verify_color(&badge.msg_color)?;

    Ok(layout)
}

pub(crate) fn for_the_badge(badge: &Badge) -> Result<Layout, BadgeError> {
    let mut layout = Layout::default();

    // Normalize text
    let font = load_font("fonts/verdana.ttf")?;
    let (label_text_norm, label_text_width) =
        get_text_dims(&font, &badge.label_text.to_uppercase(), 10.0, 1.15);
    layout.label_text_norm = label_text_norm;
    layout.label_text_width = label_text_width;
    let bold_font = load_font("fonts/verdana_bold.ttf")?;
    let (msg_text_norm, msg_text_width) =
        get_text_dims(&bold_font, &badge.msg_text.to_uppercase(), 10.0, 0.5);
    layout.msg_text_norm = msg_text_norm;
    layout.msg_text_width = msg_text_width;

    // Padding and spacing calculations
    let horiz_padding = 9.0;
    layout.badge_height = 28.0;

    // Logo padding and width
    let mut total_logo_width = 0.0;
    layout.logo_width = 0.0;
    layout.logo_padding = 0.0;
    if !badge.logo.is_empty() {
        if !badge.label_text.is_empty() {
            layout.logo_padding = 3.0;
        }
        let logo_height = 14.0;
        layout.logo_y = (layout.badge_height - logo_height) * 0.5;
        layout.logo_x = horiz_padding;
        layout.logo_width = 14.0;
        total_logo_width = layout.logo_width + layout.logo_padding;
    }

    // Label padding and width
    // This logic is something else, lemme tell ya
    layout.label_total_width = layout.label_text_width + total_logo_width + 10.0;
    if !badge.label_text.is_empty() {
        layout.label_total_width += 10.0 + (2.0 * layout.label_text_norm.len() as f32)
    } else if !badge.logo.is_empty() {
        if !badge.label_text.is_empty() {
            layout.label_total_width += 7.0;
        } else {
            layout.label_total_width -= 7.0;
        }
    } else {
        layout.label_total_width -= 11.0;
    }

    layout.label_text_x = (layout.label_total_width + total_logo_width) / 2.0;
    layout.label_text_width = layout.label_total_width - (24.0 + total_logo_width);

    // Message padding and width
    layout.msg_total_width =
        layout.msg_text_width + 20.0 + (1.5 * layout.label_text_norm.len() as f32);
    layout.msg_text_x = layout.label_total_width + (layout.msg_total_width / 2.0);
    layout.msg_text_width = layout.msg_total_width - 24.0;

    // Scale back up for the SVG
    layout.label_text_width *= 10.0;
    layout.msg_text_width *= 10.0;
    layout.label_text_x *= 10.0;
    layout.msg_text_x *= 10.0;

    // Color conversion to string
    layout.label_color = verify_color(&badge.label_color)?;
    layout.msg_color = verify_color(&badge.msg_color)?;

    Ok(layout)
}

pub(crate) fn social(badge: &Badge) -> Result<Layout, BadgeError> {
    let mut layout = Layout::default();

    // Normalize text
    let font = load_font("fonts/helvetica.ttf")?;
    let cap_label = uppercase_first_letter(&badge.label_text);
    let (label_text_norm, label_text_width) = get_text_dims(&font, &cap_label, 11.0, 0.15);
    layout.label_text_norm = label_text_norm;
    layout.label_text_width = label_text_width;
    let (msg_text_norm, msg_text_width) = get_text_dims(&font, &badge.msg_text, 11.0, 0.15);
    layout.msg_text_norm = msg_text_norm;
    layout.msg_text_width = msg_text_width;

    // Padding and spacing calculations
    let label_horiz_padding = 5.0;
    let msg_horiz_padding = 4.0;
    layout.badge_height = 20.0;

    // Logo padding and width
    let mut total_logo_width = 0.0;
    layout.logo_width = 0.0;
    layout.logo_padding = 0.0;
    if !badge.logo.is_empty() {
        if !badge.label_text.is_empty() {
            layout.logo_padding = 3.0;
        }
        let logo_height = 14.0;
        layout.logo_y = (layout.badge_height - logo_height) * 0.5;
        layout.logo_x = label_horiz_padding;
        layout.logo_width = 14.0;
        total_logo_width = layout.logo_width + layout.logo_padding;
    }

    // Label padding and width
    layout.label_rect_width =
        layout.label_text_width + total_logo_width + (2.0 * label_horiz_padding);
    layout.label_text_x = total_logo_width + (layout.label_text_width / 2.0) + label_horiz_padding;
    layout.label_total_width = layout.label_rect_width + 1.0;

    // Message padding and width
    let horiz_gutter = 6.0;
    layout.msg_rect_width = layout.msg_text_width + (2.0 * msg_horiz_padding);
    layout.msg_text_x = layout.label_rect_width + horiz_gutter + (layout.msg_rect_width / 2.0);
    layout.msg_bubble_x = layout.label_rect_width + horiz_gutter + 0.5;
    if layout.msg_text_norm.is_empty() {
        layout.msg_total_width = 0.0;
    } else {
        layout.msg_total_width = layout.msg_rect_width + horiz_gutter;
    }

    // Scale back up for the SVG
    layout.label_text_width *= 10.0;
    layout.msg_text_width *= 10.0;
    layout.label_text_x *= 10.0;
    layout.msg_text_x *= 10.0;

    // Color conversion to string
    layout.label_color = verify_color(&badge.label_color)?;
    layout.msg_color = verify_color(&badge.msg_color)?;

    println!("{:#?}", layout);

    Ok(layout)
}
