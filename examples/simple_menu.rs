extern crate spinner;

use spinner::{Menu, MenuOption};
use spinner::menu::{MenuType, MenuValue, MenuOptional};

fn main() {
    let m = Menu::new(vec![
        MenuOption("First Name".into(), MenuType::Text, MenuOptional::Optional, None),
        MenuOption("Last Name".into(), MenuType::Text, MenuOptional::Required, None),
        MenuOption("Age".into(), MenuType::Integer, MenuOptional::Optional, Some(MenuValue::Integer(1))),
        MenuOption("How much Ketchup?".into(), MenuType::Float, MenuOptional::Optional, None),
    ]);

    let results = m.display();

    println!("{:#?}", results);
}
