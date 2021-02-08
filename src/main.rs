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

use getopts::Options;
use rsbadges::Badge;
use std::env;


fn parse_project_dir_from_args() -> Result<RSBadgesOptions, BadgeError> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    // Required Args
    opts.optopt(
        "a",
        "label",
        "The text to show on the left side of the badge.",
        "<string>",
    );
    opts.optopt(
        "b",
        "label-color",
        "The background color of the left side of the badge. Supports all valid CSS formats. See \
        https://www.w3schools.com/colors/colors_picker.asp for examples.",
        "<css_color>",
    );
    opts.optopt(
        "c",
        "label-link",
        "The url to redirect to when the left side of the badge is clicked.",
        "<url>",
    );
    opts.optopt(
        "x",
        "msg",
        "The text to show on the right side of the badge.",
        "<string>",
    );
    opts.optopt(
        "y",
        "msg-color",
        "The background color of the right side of the badge.",
        "<css_color>",
    );
    opts.optopt(
        "z",
        "msg-link",
        "The url to redirect to when the right side of the badge is clicked.",
        "<url>",
    );
    opts.optopt(
        "l",
        "logo",
        "A URI reference to a logo to display in the badge.",
        "<url or local path>",
    );
    opts.optopt(
        "f",
        "save-to-svg-at",
        "The file path where this badge should be saved. File name should end in SVG \
        (but this is not enforced)",
        "<filepath>",
    );
    opts.optopt(
        "s",
        "style",
        "The Shields.io style to use during badge generation.",
        "<plastic/flat/flatsquare>",
    );
    opts.optflag(
        "o",
        "open-in-browser",
        "Flag. Display the badge in a browser tab.",
    );
    opts.optflag("h", "help", "Flag. Print arguments to console.");
    opts.optflag(
        "e",
        "embed-logo",
        "Flag. Include the specified logo data directly \
        in the badge. This prevents a URL call whenever the SVG is loaded. \
        Only works if --logo is a HTTP/HTTPS URI or a valid file path. If the \
        file cannot be successfully downloaded, rsbadges will just use the full \
        URI instead (negating this flag).",
    );
    opts.optopt(
        "",
        "badge-link",
        "The url to redirect to when any part of the badge is clicked. \
        Overwrites --label-link and --msg-link.",
        "<url>",
    );
    // See https://developer.mozilla.org/en-US/docs/Web/SVG/Element/badge_title.
    opts.optopt(
        "",
        "badge-title",
        "The title to associate with the entire badge.",
        "<string>",
    );
    opts.optopt(
        "",
        "label-title",
        "The title to associate with the left side of the badge.",
        "<string>",
    );
    opts.optopt(
        "",
        "msg-title",
        "The title to associate with the right side of the badge.",
        "<string>",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => return Err(BadgeError::BadCommandLineArgs(f.to_string())),
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", program);
        println!("{}", opts.usage(&brief));
        std::process::exit(1);
    }

    // Get the default Badge settings
    let badge_default = Badge::default();

    let label_text = matches.opt_str("label").unwrap_or(badge_default.label_text);
    let label_color = matches
        .opt_str("label-color")
        .unwrap_or(badge_default.label_color);
    let label_link = matches
        .opt_str("label-link")
        .unwrap_or(badge_default.label_link);
    let msg_text = matches.opt_str("msg").unwrap_or(badge_default.msg_text);
    let msg_color = matches
        .opt_str("msg-color")
        .unwrap_or(badge_default.msg_color);
    let msg_link = matches
        .opt_str("msg-link")
        .unwrap_or(badge_default.msg_link);

    let badge_link = matches
        .opt_str("badge-link")
        .unwrap_or(badge_default.badge_link);
    let logo = matches.opt_str("logo").unwrap_or(badge_default.logo);
    let embed_logo = matches.opt_present("e");
    let badge_title = matches
        .opt_str("badge-title")
        .unwrap_or(badge_default.badge_title);
    let label_title = matches
        .opt_str("label-title")
        .unwrap_or(badge_default.label_title);
    let msg_title = matches
        .opt_str("msg-title")
        .unwrap_or(badge_default.msg_title);

    let badge = Badge {
        label_text,
        msg_text,
        badge_link,
        label_link,
        msg_link,
        label_color,
        msg_color,
        logo,
        embed_logo,
        badge_title,
        label_title,
        msg_title,
        open_in_browser: false,
    };

    // Get our style
    let style_str = matches
        .opt_str("style")
        .unwrap_or_else(|| String::from("flat"));
    let style = match style_str.as_str() {
        "plastic" => Style::Plastic(badge),
        "flat" => Style::Flat(badge),
        "flatsquare" => Style::FlatSquare(badge),
        _ => return Err(BadgeError::InvalidStyle(style_str)),
    };

    let open_in_browser = matches.opt_present("o");
    let save_to_path = match matches.opt_str("save-to-svg-at") {
        Some(val) => val,
        None => String::from(""),
    };

    Ok(RSBadgesOptions {
        style,
        open_in_browser,
        save_to_path,
    })
}
