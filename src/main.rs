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

fn parse_project_dir_from_args() -> Result<Badge, String> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    // Required Args
    opts.optopt(
        "",
        "label",
        "the text to show on the left side of the badge",
        "build",
    );
    opts.optopt(
        "",
        "msg",
        "the text to show on the right side of the badge",
        "passed",
    );
    opts.optopt(
        "",
        "label-color",
        "the background color of the left side of the badge",
        "#555",
    );
    opts.optopt(
        "",
        "msg-color",
        "the background color of the right side of the badge",
        "#007ec6",
    );
    opts.optopt(
        "",
        "badge-link",
        "the url to redirect to when the badge is clicked",
        "",
    );
    opts.optopt(
        "",
        "label-link",
        "the url to redirect to when the left side of the badge is clicked",
        "",
    );
    opts.optopt(
        "",
        "msg-link",
        "the url to redirect to when the right side of the badge is clicked",
        "",
    );
    opts.optopt(
        "",
        "logo",
        "a URI reference to a logo to display in the badge",
        "",
    );
    opts.optopt(
        "",
        "embed-logo",
        "if the logo is specified then include the image data directly
        in the badge (this will prevent a URL fetch and may work around
             the fact that some open_in_browsers do not fetch external image
             references); only works if --logo is a HTTP/HTTPS URI or a file path",
        "false",
    );
    opts.optopt(
        "",
        "badge_title",
        "the badge title to associate with the entire badge. See https://developer.mozilla.org/en-US/docs/Web/SVG/Element/badge_title",
        "",
    );
    opts.optopt(
        "",
        "label-title",
        "the badge title to associate with the left side of the badge. See https://developer.mozilla.org/en-US/docs/Web/SVG/Element/badge_title",
        "",
    );
    opts.optopt(
        "",
        "msg-title",
        "the badge title to associate with the right side of the badge. See https://developer.mozilla.org/en-US/docs/Web/SVG/Element/badge_title",
        "",
    );
    opts.optflag("", "open-in-browser", "display the badge in a browser tab");
    opts.optflag("h", "help", "print arguments to console");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => return Err(f.to_string()),
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} FILE [options]", program);
        println!("{}", opts.usage(&brief));
        std::process::exit(1);
    }

    // Get the default Badge settings
    let badge_default = Badge::default();

    let label_text = match matches.opt_str("label") {
        Some(val) => val,
        None => badge_default.label_text,
    };
    let msg_text = match matches.opt_str("msg") {
        Some(val) => val,
        None => badge_default.msg_text,
    };
    let label_color = match matches.opt_str("label-color") {
        Some(val) => match val.parse::<css_color::Rgba>() {
            Ok(color) => color,
            Err(_) => {
                return Err(format!(
                    "Could not parse label-color argument; {} is not a valid CSS color.",
                    val
                ));
            }
        },
        None => badge_default.label_color,
    };
    let msg_color = match matches.opt_str("msg-color") {
        Some(val) => match val.parse() {
            Ok(color) => color,
            Err(_) => {
                return Err(format!(
                    "Could not parse msg-color argument; {} is not a valid CSS color.",
                    val
                ));
            }
        },
        None => badge_default.msg_color,
    };

    let badge = Badge {
        label_text,
        msg_text,
        label_color,
        msg_color,
        ..Default::default()
    };

    println!("{:#?}", badge);
    Ok(badge)
}

fn main() {
    if let Err(e) = parse_project_dir_from_args() {
        println!("{}", e);
        std::process::exit(1);
    }
}
