use rsbadges::{Badge, Style};
use std::fs;
use std::path::Path;

pub fn save_svg_to_tmp(filename: &str, svg: String) {
    let ci_path = std::env::temp_dir();
    let svg_path = ci_path.join(Path::new(filename));
    println!("Saving badge to {:#?}", svg_path);
    if let Err(c) = fs::write(svg_path, svg) {
        println!("ERROR: Could not save badge: {}", c);
    }
}

#[test]
fn create_badges_with_all_fields_populated() {
    let badge = Badge {
        label_text: String::from("version"),
        msg_text: String::from("1.2.3"),
        label_color: String::from("#555"),
        msg_color: String::from("#007ec6"),
        ..Badge::default()
    };
    let mut svg = match Style::Flat(badge.clone()).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge.svg", svg);
    svg = match Style::FlatSquare(badge.clone()).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_square_badge.svg", svg);
    svg = match Style::Plastic(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("plastic_badge.svg", svg);
}

#[test]
fn create_badge_with_logo() {
    let badge = Badge {
        label_text: String::from("version"),
        msg_text: String::from("1.2.3"),
        logo: String::from("https://simpleicons.org/icons/slack.svg"),
        ..Badge::default()
    };
    let svg = match Style::Flat(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge_logo.svg", svg);
}

#[test]
fn create_badge_embed_logo() {
    let badge = Badge {
        logo: String::from("https://simpleicons.org/icons/slack.svg"),
        embed_logo: true,
        ..Badge::default()
    };
    let svg = match Style::Flat(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge_logo_embedded.svg", svg);
}

#[test]
fn create_badge_chinese_characters() {
    let badge = Badge {
        label_text: String::from("版"),
        msg_text: String::from("不知道"),
        ..Badge::default()
    };
    let svg = match Style::Flat(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge_chinese.svg", svg);
}

#[test]
fn create_badge_arabic_characters() {
    let badge = Badge {
        label_text: String::from("اختبار"),
        msg_text: String::from("انا لا اعرف"),
        ..Badge::default()
    };
    let svg = match Style::Flat(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge_arabic.svg", svg);
}

#[test]
fn create_badge_metal() {
    let badge = Badge {
        label_text: String::from("röck döts"),
        msg_text: String::from("very metal indeed"),
        ..Badge::default()
    };
    let svg = match Style::Flat(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge_metal.svg", svg);
}

#[test]
fn create_badge_badge_link() {
    let badge = Badge {
        badge_link: String::from("http://www.crates.io"),
        ..Badge::default()
    };
    let svg = match Style::Flat(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge_full_link.svg", svg);
}

#[test]
fn create_badge_with_separate_label_msg_links() {
    let badge = Badge {
        label_link: String::from("http://www.tangramvision.com"),
        msg_link: String::from("http://www.crates.io"),
        ..Badge::default()
    };
    let svg = match Style::Flat(badge).generate_svg() {
        Ok(f) => f,
        Err(_) => unreachable!(),
    };
    save_svg_to_tmp("flat_badge_two_links.svg", svg);
}
