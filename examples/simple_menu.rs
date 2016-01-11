extern crate spinner;

use spinner::{Menu, MenuOption};
use spinner::menu::{MenuType, MenuValue};

fn main() {
    let m = Menu::new(vec![
        MenuOption("First Name".into(), MenuType::Text(false), None),
        MenuOption("Last Name".into(), MenuType::Text(true), None),
        MenuOption("Age".into(), MenuType::Integer(true), Some(MenuValue::Integer(1))),
        MenuOption("How much Ketchup?".into(), MenuType::Float(true), None),
    ]);

    let results = m.display();

    println!("{:#?}", results);
}
