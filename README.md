# RSBadges

## Create shields.io-like badges from the comfort and safety of Rust

| Flat                             | Flat Square                                   | Plastic                               | ForTheBadge                                 | Social                                      |
|----------------------------------|-----------------------------------------------|---------------------------------------|---------------------------------------------|---------------------------------------------|
| ![flat](/assets/flat_badge.svg)  | ![flat_square](/assets/flat_square_badge.svg) | ![plastic](/assets/plastic_badge.svg) | ![for_the_badge](/assets/for_the_badge.svg) | ![social](/assets/social_logo_embedded.svg) |


RSBadges is a Rust-friendly badge generator. The interface strives to be minimal
while still providing a feature-rich API. Both the label (the left side) and the
message (the right side) of the badge can be customized fully, with the ability to

- Set text
- Set color using any valid CSS color code
- Embed a link into each side or a link for the whole badge
- Add a logo (in SVG format) from a local source or a URL
- Embed that logo's data into the badge directly
- Set the style of badge, as described in [Shields.io](http://shields.io)

RSBadges can be used as an API or a command line interface (CLI). See the docs for more
details on arguments and capabilities.

-----

# API

First, instantiate a Badge struct to set all of the generic options for a badge SVG.
This fully-populated Badge is then wrapped in a Style enum, which indicates which
style of badge to eventually generate.

```
use rsbadges::{Badge, Style};
let badge = Badge {
    label_text: String::from("Custom_label"),
    msg_text: String::from("Custom_msg"),
    label_color: String::from("red"),
    ..Badge::default()
};
// Create a plastic badge using the data created above.
let badge_style = Style::Plastic(badge);
```

Badge and Style together are sufficient to
tell RSBadges how to construct the right badge, which it does through `generate_svg()`:

```
let badge_svg = badge_style.generate_svg().unwrap();
```

The resulting SVG String can be saved to file using the convenience function `save_svg()`:

```
rsbadges::save_svg("~/Downloads/badge.svg", &badge_svg);
```

See the documentation for the Badge and Style types for more.

-----

# CLI

The CLI features all of the customization options from the API, along with a
few quality-of-life improvements for command line use and evaluation, such as

- Opening a created badge SVG in browser after creation
- Specifying a save directory for the SVG

Valid argument formats match those found in the API (see [Badge]).
Don't worry if you get it wrong; RSBadges will let you know.

| Short      | Long                                                      | Default
| ---------  | ------------------------------------                      | -------
| `-a`       | `--label <string>`                                        | "test"
| `-b`       | `--label-color <css_color>`                               | "#555"
| `-c`       | `--label-link <url>`                                      | ""
| `-x`       | `--msg <string>`                                          | "test"
| `-y`       | `--msg-color <css_color>`                                 | "#007ec6"
| `-z`       | `--msg-link <url>`                                        | ""
| `-l`       | `--logo <url or local path>`                              | ""
| `-f`       | `--save-to-svg-at <filepath/file.svg>`                    | ""
| `-s`       | `--style <plastic,flat,flatsquare,forthebadge,social>`    | "flat"
| `-o`       | `--open-in-browser`                                       | false
| `-h`       | `--help`                                                  | false
| `-e`       | `--embed-logo`                                            | false

 Run the CLI with the `-h` flag to see all possible arguments and flags.
