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
use super::text_helper::*;

pub fn plastic(badge: &Badge) -> Layout {
    let mut layout = Layout::default();

    // Normalize text
    let font = load_regular_font();
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
    layout.label_color = color_to_string(badge.label_color);
    layout.msg_color = color_to_string(badge.msg_color);

    layout
}

pub fn flat_or_square(badge: &Badge) -> Layout {
    let mut layout = Layout::default();

    // Normalize text
    let font = load_regular_font();
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
        let logo_height = 14.0; // hardcoded
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
    layout.label_color = color_to_string(badge.label_color);
    layout.msg_color = color_to_string(badge.msg_color);

    layout
}

// fn for_the_badge() {

// TODO: MSG FONT SHOULD BE BOLD
// // Normalize text
// let font = text_helper::load_regular_font();
// let bold_font = load_bold_font();
// layout.label_text_width = text_helper::get_text_dims(&font, &badge.label_text, 11.0, 0.8).0;
// layout.msg_text_width = text_helper::get_text_dims(&font, &badge.msg_text, 11.0, 0.8).0;

// // Padding and spacing calculations
// let horiz_padding = match flavor {
//     Flavor::ForTheBadge => 9.0,
//     _ => 5.0,
// };
// layout.badge_height = match flavor {
//     Flavor::ForTheBadge => 28.0,
//     Flavor::Plastic => 18.0,
//     Flavor::Flat => 20.0,
//     Flavor::FlatSquare => 20.0,
//     // Flavor::Social => 20.0,
// };

// TODO: Complete for ForTheBadge
// // Label padding and width
// layout.label_total_width = layout.label_text_width + 10.0;
// if !badge.label_text.is_empty() {
//     layout.label_total_width += 10.0 + (2.0 * badge.label_text.len() as f32);
// } else if !badge.logo.is_empty() {
//     if !badge.label_text.is_empty() {
//         layout.label_total_width += 7.0;
//     } else {
//         layout.label_total_width -= 7.0;
//     }
// } else {
//     layout.label_total_width -= 11.0;
// }

// layout.label_text_x = (layout.label_total_width + total_logo_width) / 2.0;
// layout.label_text_width =
//     layout.label_total_width - (24.0 + total_logo_width);

// // Message padding and width
// layout.msg_total_width = layout.msg_text_width * 1.5 + 20.0;
// let left_width = 0;
// let

// }

// if badge.label_text.is_empty() {
//     if !badge.logo.is_empty() {
//         msg_text_margin += layout.logo_width + horiz_padding;
//     } else {
//         msg_text_margin += 1.0;
//     }
// }

// layout.msg_total_width = layout.msg_text_width + (2.0 * horiz_padding);
// if !badge.logo.is_empty() && badge.label_text.is_empty() {
//     layout.msg_total_width += layout.logo_width + horiz_padding - 1.0;
// }

// layout.msg_text_x =
//     msg_text_margin + (0.5 * layout.msg_text_width) + horiz_padding;
// }
