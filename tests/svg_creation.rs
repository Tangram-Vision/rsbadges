use rsbadges::Badge;
use rsbadges::Style;
use std::fs;
use std::path::Path;

fn save_badge_to_tmp(filename: &str, svg: String) {
    let ci_path = std::env::temp_dir();
    let svg_path = ci_path.join(Path::new(filename));
    println!("Saving badge to {:#?}", svg_path);
    if let Err(c) = fs::write(svg_path, svg) {
        println!("ERROR: Could not save badge: {}", c);
    }
}

#[test]
fn create_badges() {
    let badge = Badge {
        label_text: String::from("version"),
        msg_text: String::from("1.2.3"),
        label_color: "#555".parse().unwrap(),
        msg_color: "#007ec6".parse().unwrap(),
        ..Badge::default()
    };

    save_badge_to_tmp("flat_badge.svg", Style::Flat(badge.clone()).generate_svg());
    save_badge_to_tmp(
        "flat_square_badge.svg",
        Style::FlatSquare(badge.clone()).generate_svg(),
    );
    save_badge_to_tmp("plastic_badge.svg", Style::Plastic(badge).generate_svg());
}

#[test]
fn create_badges_with_logos() {
    let badge = Badge {
        label_text: String::from("version"),
        msg_text: String::from("1.2.3"),
        label_color: "#555".parse().unwrap(),
        msg_color: "#007ec6".parse().unwrap(),
        logo: String::from("https://simpleicons.org/icons/slack.svg"),
        ..Badge::default()
    };

    save_badge_to_tmp("flat_logo.svg", Style::Flat(badge.clone()).generate_svg());
    save_badge_to_tmp(
        "flat_square_logo.svg",
        Style::FlatSquare(badge.clone()).generate_svg(),
    );
    save_badge_to_tmp("plastic_logo.svg", Style::Plastic(badge).generate_svg());
}

#[test]
fn create_badges_with_logo_embedded() {
    let badge = Badge {
        label_text: String::from("version"),
        msg_text: String::from("1.2.3"),
        label_color: "#555".parse().unwrap(),
        msg_color: "#007ec6".parse().unwrap(),
        logo: String::from("https://simpleicons.org/icons/slack.svg"),
        embed_logo: true,
        ..Badge::default()
    };

    save_badge_to_tmp(
        "flat_logo_embed.svg",
        Style::Flat(badge.clone()).generate_svg(),
    );
    save_badge_to_tmp(
        "flat_square_logo_embed.svg",
        Style::FlatSquare(badge.clone()).generate_svg(),
    );
    save_badge_to_tmp(
        "plastic_logo_embed.svg",
        Style::Plastic(badge).generate_svg(),
    );
}

#[test]
fn create_badge_chinese_characters() {
    let badge = Badge {
        label_text: String::from("版"),
        msg_text: String::from("不知道"),
        label_color: "#555".parse().unwrap(),
        msg_color: "#007ec6".parse().unwrap(),
        ..Badge::default()
    };

    save_badge_to_tmp(
        "flat_chinese.svg",
        Style::Flat(badge.clone()).generate_svg(),
    );
    save_badge_to_tmp(
        "flat_square_chinese.svg",
        Style::FlatSquare(badge.clone()).generate_svg(),
    );
    save_badge_to_tmp("plastic_chinese.svg", Style::Plastic(badge).generate_svg());
}

#[test]
fn create_badge_arabic_characters() {
    let badge = Badge {
        label_text: String::from("اختبار"),
        msg_text: String::from("انا لا اعرف"),
        label_color: "#555".parse().unwrap(),
        msg_color: "#007ec6".parse().unwrap(),
        ..Badge::default()
    };

    save_badge_to_tmp("flat_arabic.svg", Style::Flat(badge.clone()).generate_svg());
    save_badge_to_tmp(
        "flat_square_arabic.svg",
        Style::FlatSquare(badge.clone()).generate_svg(),
    );
    save_badge_to_tmp("plastic_arabic.svg", Style::Plastic(badge).generate_svg());
}

#[test]
fn create_badge_metal() {
    let badge = Badge {
        label_text: String::from("röck döts"),
        msg_text: String::from("metal"),
        label_color: "#555".parse().unwrap(),
        msg_color: "#007ec6".parse().unwrap(),
        ..Badge::default()
    };

    save_badge_to_tmp("flat_metal.svg", Style::Flat(badge.clone()).generate_svg());
    save_badge_to_tmp(
        "flat_square_metal.svg",
        Style::FlatSquare(badge.clone()).generate_svg(),
    );
    save_badge_to_tmp("plastic_metal.svg", Style::Plastic(badge).generate_svg());
}

#[test]
fn create_badges_badge_link() {
    let badge = Badge {
        label_text: String::from("version"),
        msg_text: String::from("1.2.3"),
        label_color: "#555".parse().unwrap(),
        msg_color: "#007ec6".parse().unwrap(),
        badge_link: String::from("http://www.tangramvision.com"),
        ..Badge::default()
    };

    save_badge_to_tmp("flat_link.svg", Style::Flat(badge.clone()).generate_svg());
    save_badge_to_tmp(
        "flat_square_link.svg",
        Style::FlatSquare(badge.clone()).generate_svg(),
    );
    save_badge_to_tmp("plastic_link.svg", Style::Plastic(badge).generate_svg());
}

#[test]
fn create_badges_with_label_msg_links() {
    let badge = Badge {
        label_text: String::from("version"),
        msg_text: String::from("1.2.3"),
        label_color: "#555".parse().unwrap(),
        msg_color: "#007ec6".parse().unwrap(),
        label_link: String::from("http://www.tangramvision.com"),
        msg_link: String::from("http://www.google.com"),
        ..Badge::default()
    };

    save_badge_to_tmp("flat_links.svg", Style::Flat(badge.clone()).generate_svg());
    save_badge_to_tmp(
        "flat_square_links.svg",
        Style::FlatSquare(badge.clone()).generate_svg(),
    );
    save_badge_to_tmp("plastic_links.svg", Style::Plastic(badge).generate_svg());
}
