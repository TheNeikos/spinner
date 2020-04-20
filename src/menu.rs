use std::io::{stdin, stdout, BufRead, Read, Write};
use std::fmt::{Display, Formatter, Error as FmtError};
use std::str::FromStr;

use ansi_term::Style;
use ansi_term::Colour::Red;

/// An enum specifying if a given field is optional or not
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MenuOptional {
    /// This will make the field optional
    Optional,
    /// This will make the field required
    Required,
}

/// An enum specifying the type of information the user should put in.
///
/// This is then later checked against by trying to convert into this type.
/// If it fails the user is asked to type in again.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MenuType {
    /// Input should be type Text
    Text,
    /// Input should be type Integer
    Integer,
    /// Input should be type Float
    Float,
}

/// The value of the given menu, when given as an argument to the constructor
/// this will be used as the default value. When returned after displaying
/// the menu it will contain the information input by the user.
#[derive(Clone, Debug)]
pub enum MenuValue {
    /// The value of a Text MenuOption
    Text(String),
    /// The value of an Integer MenuOption
    Integer(i64),
    /// The value of a Float MenuOption
    Float(f64),
}

impl Display for MenuValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            &MenuValue::Text(ref s) => s.fmt(f),
            &MenuValue::Integer(ref s) => s.fmt(f),
            &MenuValue::Float(ref s) => s.fmt(f)
        }
    }
}

/// An individual MenuOption, check out the crate documentation on how to use it
#[derive(Clone, Debug)]
pub struct MenuOption(pub String, pub MenuType, pub MenuOptional, pub Option<MenuValue>);

impl MenuOption {
    fn set_string(&mut self, s: String) {
        self.3 = Some(MenuValue::Text(s));
    }

    fn set_int(&mut self, i: i64) {
        self.3 = Some(MenuValue::Integer(i));
    }

    fn set_float(&mut self, f: f64) {
        self.3 = Some(MenuValue::Float(f));
    }

    /// Get the string if there is one, will panic if MenuType is not a string
    ///
    /// # Panic
    ///
    /// This will panic if the types do not match
    pub fn get_string(mut self) -> Option<String> {
        match self.3.take() {
            Some(MenuValue::Text(s)) => Some(s),
            None => None,
            a => panic!("Tried to take a String out of a menu option that is a {:?}", a)
        }
    }

    /// Get the integer if there is one, will panic if MenuType is not an
    /// integer
    ///
    /// # Panic
    ///
    /// This will panic if the types do not match
    pub fn get_int(mut self) -> Option<i64> {
        match self.3.take() {
            Some(MenuValue::Integer(s)) => Some(s),
            None => None,
            a => panic!("Tried to take a Integer out of a menu option that is a {:?}", a)
        }
    }

    /// Get the float if there is one, will panic if MenuType is not a float
    ///
    /// # Panic
    ///
    /// This will panic if the types do not match
    pub fn get_float(mut self) -> Option<f64> {
        match self.3.take() {
            Some(MenuValue::Float(s)) => Some(s),
            None => None,
            a => panic!("Tried to take a Float out of a menu option that is a {:?}", a)
        }
    }


    fn is_optional(&self) -> bool {
        match self.2 {
            MenuOptional::Optional => true,
            MenuOptional::Required => false
        }
    }

    fn has_value(&self) -> bool {
        self.3.is_some()
    }

    fn set(&mut self, s: String) -> Result<(), ()> {
        match self.1 {
            MenuType::Text => {
                let m: &[_] = &['\n', '\r'];
                self.set_string(s.trim_right_matches(m).into());
                Ok(())
            }
            MenuType::Integer => {
                let try = i64::from_str(s.trim());
                if try.is_err() {
                    return Err(());
                }
                self.set_int(try.unwrap());
                Ok(())
            }
            MenuType::Float => {
                let try = f64::from_str(s.trim());
                if try.is_err() {
                    return Err(());
                }
                self.set_float(try.unwrap());
                Ok(())
            }
        }
    }

    fn get_type_name(&self) -> &'static str {
        match self.1 {
            MenuType::Text => "Text",
            MenuType::Integer => "Integer",
            MenuType::Float => "Float",
        }
    }
}

/// A menu that has not yet been displayed
#[derive(Clone, Debug)]
pub struct Menu {
    items: Vec<MenuOption>,
}

impl Menu {
    /// Construct a new menu using the given MenuOptions
    pub fn new(i: Vec<MenuOption>) -> Self {
        Menu {
            items: i,
        }
    }

    /// Consume the menu and return a Vector of MenuOptions with the new values
    /// inserted.
    pub fn display(mut self) -> Vec<MenuOption> {
        let mut i = 0;
        let max = self.items.len();
        //{
        //    // Flush stdin, so previous does not get put in here
        //    let mut b = Vec::new();
        //    let _ = stdin().read(&mut b).unwrap();
        //}
        while i < max {
            let ref mut item = self.items[i];
            print!("{} {}expecting {}: ", Style::new().bold().paint(&item.0[..]), {
                if let Some(ref s) = item.3 {
                    format!("({}default: \"{}\") ", {
                        if item.is_optional() {
                            "Optional, ".into()
                        } else {
                            String::new()
                        }
                    }, s)
                } else {
                    if item.is_optional() {
                        "(Optional) ".into()
                    } else {
                        String::new()
                    }
                }
            }, Style::new().italic().paint(item.get_type_name()));
            stdout().flush().unwrap();
            let mut buffer = String::new();
            // TODO: Think of a way to not unwrap here, result?
            let _ = stdin().read_line(&mut buffer).unwrap();

            let empty = buffer.chars().all(char::is_whitespace);
            if empty && !item.is_optional() && !item.has_value() {
                println!("{}", Red.paint("This item is not optional, please enter a value."));
                continue;
            } else if empty {
                // We do not remove the default value
                i = i + 1;
                continue;
            }

            if item.set(buffer).is_err() {
                println!("{}", Red.paint("You have entered the wrong type of information."));
                continue;
            }
            i = i + 1;
        }
        self.items
    }
}
