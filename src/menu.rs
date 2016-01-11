use std::io::{stdin, stdout, BufRead, Read, Write};
use std::fmt::{Display, Formatter, Error as FmtError};
use std::str::FromStr;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MenuType {
    Text(bool),
    Integer(bool),
    Float(bool)
}

#[derive(Clone, Debug)]
pub enum MenuValue {
    Text(String),
    Integer(i64),
    Float(f64)
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

#[derive(Clone, Debug)]
pub struct MenuOption(pub String, pub MenuType, pub Option<MenuValue>);

impl MenuOption {
    fn set_string(&mut self, s: String) {
        self.2 = Some(MenuValue::Text(s));
    }

    fn set_int(&mut self, i: i64) {
        self.2 = Some(MenuValue::Integer(i));
    }

    fn set_float(&mut self, f: f64) {
        self.2 = Some(MenuValue::Float(f));
    }

    pub fn get_string(mut self) -> Option<String> {
        match self.2.take() {
            Some(MenuValue::Text(s)) => Some(s),
            None => None,
            a => panic!("Tried to take a String out of a menu option that is a {:?}", a)
        }
    }

    pub fn get_int(mut self) -> Option<i64> {
        match self.2.take() {
            Some(MenuValue::Integer(s)) => Some(s),
            None => None,
            a => panic!("Tried to take a Integer out of a menu option that is a {:?}", a)
        }
    }

    pub fn get_float(mut self) -> Option<f64> {
        match self.2.take() {
            Some(MenuValue::Float(s)) => Some(s),
            None => None,
            a => panic!("Tried to take a Float out of a menu option that is a {:?}", a)
        }
    }

    pub fn is_optional(&self) -> bool {
        match self.1 {
            MenuType::Text(a) => a,
            MenuType::Integer(a) => a,
            MenuType::Float(a) => a,
        }
    }

    pub fn set(&mut self, s: String) -> Result<(), ()>{
        match self.1 {
            MenuType::Text(_) => {
                let m: &[_] = &['\n', '\r'];
                self.set_string(s.trim_right_matches(m).into());
                Ok(())
            },
            MenuType::Integer(_) => {
                let try = i64::from_str(s.trim());
                if try.is_err() {
                    return Err(());
                }
                self.set_int(try.unwrap());
                Ok(())
            },
            MenuType::Float(_) => {
                let try = f64::from_str(s.trim());
                if try.is_err() {
                    return Err(());
                }
                self.set_float(try.unwrap());
                Ok(())
            },
        }
    }

    pub fn get_type_name(&self) -> &'static str {
        match self.1 {
            MenuType::Text(_) => "Text",
            MenuType::Integer(_) => "Integer",
            MenuType::Float(_) => "Float",
        }
    }
}

pub struct Menu {
    items: Vec<MenuOption>,
}

impl Menu {
    pub fn new(i: Vec<MenuOption>) -> Self {
        Menu {
            items: i,
        }
    }

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
            print!("{} {}, expecting {}: ", item.0, {
                if item.is_optional() {
                    if let Some(ref s) = item.2 {
                        format!("(Optional, default: \"{}\")", s)
                    } else {
                        "(Optional)".into()
                    }
                } else {
                    String::new()
                }
            }, item.get_type_name());
            stdout().flush().unwrap();
            let mut buffer = String::new();
            // TODO: Think of a way to not unwrap here, result?
            let _ = stdin().read_line(&mut buffer).unwrap();

            let empty = buffer.chars().all(char::is_whitespace);
            if empty && !item.is_optional() {
                println!("This item is not optional, please enter a value.");
                continue;
            } else if empty {
                // We do not remove the default value
                i = i + 1;
                continue;
            }

            if item.set(buffer).is_err() {
                println!("You have entered the wrong type of information.");
                continue;
            }
            i = i + 1;
        }
        self.items
    }
}
