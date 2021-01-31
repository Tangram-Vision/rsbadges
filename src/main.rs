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
             the fact that some browsers do not fetch external image 
             references); only works if --logo is a HTTP/HTTPS URI or a file path",
        "",
    );
    opts.optopt(
        "t",
        "title",
        "the title to associate with the entire badge. See https://developer.mozilla.org/en-US/docs/Web/SVG/Element/title",
        "",
    );
    opts.optopt(
        "lt",
        "left-title",
        "the title to associate with the left side of the badge. See https://developer.mozilla.org/en-US/docs/Web/SVG/Element/title",
        "",
    );
    opts.optopt(
        "rt",
        "right-title",
        "the title to associate with the right side of the badge. See https://developer.mozilla.org/en-US/docs/Web/SVG/Element/title",
        "",
    );
    opts.optopt(
        "browser",
        "browser",
        "display the badge in a browser tab",
        "",
    );

    // parser.add_argument(
    //     '--deja-vu-sans-path',
    //     default=None,
    //     help='the path to the ttf font file containing DejaVu Sans. If not ' +
    //     'present on your system, you can download it from ' +
    //     'https://www.fontsquirrel.com/fonts/dejavu-sans')

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => return Err(f.to_string()),
    };

    let home_dir = match matches.opt_str("d") {
        Some(d) => d,
        None => {
            let brief = format!("Usage: {} FILE [options]", program);
            return Err(opts.usage(&brief));
        }
    };
    Ok(home_dir)
}

fn main() {}
