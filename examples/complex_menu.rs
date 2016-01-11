extern crate spinner;

use spinner::{Menu, MenuOption};
use spinner::menu::{MenuType, MenuValue, MenuOptional};

fn main() {
    println!("Welcome to the Rust TipCalculator MKI");
    let m = Menu::new(vec![
        MenuOption("Bill".into(), MenuType::Float,
                   MenuOptional::Required, None),
        MenuOption("Tip Percentage (eg. 10 for 10%)".into(), MenuType::Integer,
                   MenuOptional::Required, Some(MenuValue::Integer(10))),
        MenuOption("Number of People".into(), MenuType::Integer,
                   MenuOptional::Required, Some(MenuValue::Integer(1))),
    ]);

    let mut results = m.display();

    let ppl = results.pop().unwrap().get_int().unwrap();
    let tip_p = results.pop().unwrap().get_int().unwrap();
    let bill = results.pop().unwrap().get_float().unwrap();

    let tip = bill * (tip_p as f64/100f64);
    let total = bill + tip;

    if ppl < 1 {
        println!("You need at least one person paying.");
        return;
    }

    println!("{} pay {}, the tip is {}", if ppl > 1 { "Each of you" } else { "You" },
             total/ppl as f64, tip);

}
