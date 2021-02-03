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
use std::env;
use std::process::Command; // For git stuff

fn parse_project_dir_from_args() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    // Required Args
    opts.reqopt(
        "l",
        "left-text",
        "the text to show on the left side of the badge",
        "build",
    );
    opts.reqopt(
        "r",
        "right-text",
        "the text to show on the right side of the badge",
        "passed",
    );
    opts.reqopt(
        "lcol",
        "left-color",
        "the background color of the left side of the badge",
        "#555555",
    );
    opts.reqopt(
        "rcol",
        "right-color",
        "the background color of the right side of the badge",
        "#00ff00",
    );

    // Optional Args
    opts.optopt(
        "link",
        "link",
        "the url to redirect to when the badge is clicked",
        "",
    );
    opts.optopt(
        "ll",
        "left-link",
        "the url to redirect to when the left side of the badge is clicked",
        "",
    );
    opts.optopt(
        "rl",
        "right-link",
        "the url to redirect to when the right side of the badge is clicked",
        "",
    );
    opts.optopt(
        "logo",
        "logo",
        "a URI reference to a logo to display in the badge",
        "",
    );
    opts.optopt(
        "el",
        "embed-logo",
        "if the logo is specified then include the image data directly
        in the badge (this will prevent a URL fetch and may work around
             the fact that some open_in_browsers do not fetch external image
             references); only works if --logo is a HTTP/HTTPS URI or a file path",
        "",
    );
    opts.optopt(
        "t",
        "badge_title",
        "the badge_title to associate with the entire badge. See https://developer.mozilla.org/en-US/docs/Web/SVG/Element/badge_title",
        "",
    );
    opts.optopt(
        "lt",
        "left-badge_title",
        "the badge_title to associate with the left side of the badge. See https://developer.mozilla.org/en-US/docs/Web/SVG/Element/badge_title",
        "",
    );
    opts.optopt(
        "rt",
        "right-badge_title",
        "the badge_title to associate with the right side of the badge. See https://developer.mozilla.org/en-US/docs/Web/SVG/Element/badge_title",
        "",
    );
    opts.optopt(
        "open_in_browser",
        "open_in_browser",
        "display the badge in a open_in_browser tab",
        "",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => return Err(f.to_string()),
    };

    let label_text = match_arg("l", &matches, &program, &opts);
    let msg_text = match_arg("r", &matches, &program, &opts);

    // Tricky: Grab the color
    // We have to use
    // `color_str.parse::<css_color::Rgba>()?;`
    let label_color_arg = match_arg("lcol", &matches, &program, &opts);
    let msg_color_arg = match_arg("rcol", &matches, &program, &opts);
    // first to make sure that the color passed is valid.

    Ok(String::from(""))
}

fn match_arg(
    arg: &str,
    matches: &getopts::Matches,
    program: &str,
    opts: &getopts::Options,
) -> Result<String, String> {
    match matches.opt_str(&arg) {
        Some(d) => Ok(d),
        None => {
            let brief = format!("Usage: {} FILE [options]", program);
            return Err(opts.usage(&brief));
        }
    }
}

fn main() {}
