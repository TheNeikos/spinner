extern crate spinner;

use spinner::menu::{MenuOptional, MenuType, MenuValue};
use spinner::{Menu, MenuOption};

fn main() {
    let m = Menu::new(vec![
        MenuOption(
            "First Name".into(),
            MenuType::Text,
            MenuOptional::Required,
            None,
        ),
        MenuOption(
            "Last Name".into(),
            MenuType::Text,
            MenuOptional::Optional,
            None,
        ),
        MenuOption(
            "Age".into(),
            MenuType::Integer,
            MenuOptional::Required,
            Some(MenuValue::Integer(1)),
        ),
        MenuOption(
            "How much Ketchup?".into(),
            MenuType::Float,
            MenuOptional::Required,
            None,
        ),
    ]);

    let results = m.display();

    println!("{:#?}", results);
}
