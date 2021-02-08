use rsbadges::{Badge, BadgeError, Style};

#[test]
fn error_color_not_valid() {
    let badge = Badge {
        label_color: String::from("#t"),
        ..Badge::default()
    };
    match Style::Flat(badge.clone()).generate_svg() {
        Err(BadgeError::ColorNotValid(_)) => {}
        _ => unreachable!(),
    }
    match Style::FlatSquare(badge.clone()).generate_svg() {
        Err(BadgeError::ColorNotValid(_)) => {}
        _ => unreachable!(),
    }
    match Style::Plastic(badge).generate_svg() {
        Err(BadgeError::ColorNotValid(_)) => {}
        _ => unreachable!(),
    }

    let badge = Badge {
        msg_color: String::from("rgb(300.0)"),
        ..Badge::default()
    };
    match Style::Flat(badge.clone()).generate_svg() {
        Err(BadgeError::ColorNotValid(_)) => {}
        _ => unreachable!(),
    }
    match Style::FlatSquare(badge.clone()).generate_svg() {
        Err(BadgeError::ColorNotValid(_)) => {}
        _ => unreachable!(),
    }
    match Style::Plastic(badge).generate_svg() {
        Err(BadgeError::ColorNotValid(_)) => {}
        _ => unreachable!(),
    }
}

#[test]
fn error_cannot_embed_logo() {
    let badge = Badge {
        logo: String::from("bad_dir_nothing_here"),
        embed_logo: true,
        ..Badge::default()
    };
    // Flat and FlatSquare use the same code here, so no
    // need to test twice
    match Style::Flat(badge.clone()).generate_svg() {
        Err(BadgeError::CannotEmbedLogo(_)) => {}
        _ => unreachable!(),
    }
    match Style::Plastic(badge).generate_svg() {
        Err(BadgeError::CannotEmbedLogo(_)) => {}
        _ => unreachable!(),
    }
}
