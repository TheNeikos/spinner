//! This library is useful for console applications with long running processes.
//! You create a `Spinner` and then update it however you see fit. Since this
//! happens asynchronously, your user will not be left in the dark about what
//! your app is doing.
//!
//!
//! There are two parts to Spinner, one part is the spinner itself, the other a
//! simple interface to build menus.
//!
//!
//! ### Spinners
//!
//! To use a Spinner simply go and create one using the SpinnerBuilder:
//!
//! ```rust
//! use spinner::SpinnerBuilder;
//! let sp = SpinnerBuilder::new("Long Running operation, please wait...".into()).start();
//! ```
//!
//! Will inform the user that your app currently is doing background processing.
//! `sp` is a `SpinnerHandle`, through which you can tell the user for example how
//! far along the process you are, or perhaps a message in between.
//!
//! ```
//! use spinner::SpinnerBuilder;
//! let sp = SpinnerBuilder::new("Long Running operation, please wait...".into()).start();
//! sp.message("Updating...".into());
//! # let (i, max) = (0usize, 3usize);
//! sp.update(format!("Finished {} out of {}.", i, max));
//! ```
//!
//! #### Customizing
//!
//! A spinner can be customized in three ways:
//!
//! - The `step` duration, which is the 'refresh' period of the message.
//! - The `format`, how a given string is printed, due to limitations this is
//!     done through a closure, but it also allows more special formatting than
//!     just a format string.
//! - The `spinner` itself, which is the list of characters that change every
//!     step.
//!
//!
//! ### Menus
//!
//! Menus are simple, type checked ways to ask the user for information.
//!
//! A simple menu might look like this:
//!
//! ```no_run
//! use spinner::menu::*;
//! let m = Menu::new(vec![
//!     MenuOption("First Name".into(), MenuType::Text, MenuOptional::Optional, None),
//!     MenuOption("Last Name".into(), MenuType::Text, MenuOptional::Required, None),
//!     MenuOption("Age".into(), MenuType::Integer, MenuOptional::Optional, Some(MenuValue::Integer(1))),
//!     MenuOption("How much Ketchup?".into(), MenuType::Float, MenuOptional::Optional, None),
//! ]);
//!
//! let results = m.display();
//! ```
//!
//! In results will then be an array of `MenuValue`, which can then be retrieved by
//! either `get_{string,int,float}`, calling one of these on the wrong type will
//! **panic!**. So be careful to take out the correct value out of the correct menu
//! questions.
//!
//! #### MenuOption
//!
//! A MenuOption is a NewType. It consists of a string which will constitute the
//! question being presented to the user. Then a MenuType, telling the checker what
//! input is expected. If you need something else use `MenuType::Text` and work with
//! that. You also have to tell whether that input is optional or not.
//! (true=optional, false=not optional). At last, an `Option<MenuValue>` which allows
//! you to put in either `None`, for no default value or `Some<MenuValue>` which
//! will be used if the user inputs nothing.

#![deny(
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

extern crate ansi_term;
extern crate term;

/// Module defining the menu part of this crate
pub mod menu;

pub use menu::Menu;
pub use menu::MenuOption;

use std::io::{stdout, Write};
use std::sync::mpsc::{channel, Receiver, SendError, Sender, TryRecvError};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Debug)]
enum SpinnerMessage {
    Status(String),
    Message(String),
}

type FormatFn = dyn Fn(&str, &str) -> String + Send + 'static;

/// A possible string for the spinner, check out the kirby example for a
/// possible use case.
pub static DANCING_KIRBY: [&'static str; 8] = [
    "(>'-')>", "<('-'<)", "^('-')^", "<('-'<)", "(>'-')>", "<('-'<)", "^('-')^", "<('-'<)",
];

struct Spinner {
    status: String,
    types: Vec<&'static str>,
    rx: Receiver<SpinnerMessage>,
    custom_out: Option<Box<FormatFn>>,
    step: Duration,
}

impl Spinner {
    fn start(sp: Spinner, tx: Sender<SpinnerMessage>) -> SpinnerHandle {
        let th = thread::spawn(move || {
            let mut sp = sp;
            for ttype in sp.types.iter().cycle() {
                let mut msg = None;
                let mut should_disc = false;
                loop {
                    match sp.rx.try_recv() {
                        Ok(ms) => match ms {
                            SpinnerMessage::Status(st) => sp.status = st,
                            SpinnerMessage::Message(st) => msg = Some(st),
                        },
                        Err(TryRecvError::Empty) => break,
                        Err(TryRecvError::Disconnected) => {
                            should_disc = true;
                            break;
                        }
                    };
                }

                if let Some(m) = msg {
                    println!("\n{}", m);
                }

                if should_disc {
                    break;
                }

                if let Some(mut t) = term::stdout() {
                    t.carriage_return().unwrap();
                    t.delete_line().unwrap();
                }

                if let Some(ref cl) = sp.custom_out {
                    print!("{}", cl(ttype, &sp.status[..]));
                } else {
                    print!("{} {}", ttype, sp.status);
                }
                {
                    let stdout = stdout();
                    stdout.lock().flush().unwrap();
                }
                thread::sleep(sp.step)
            }
        });

        SpinnerHandle {
            send: tx,
            handle: Some(th),
        }
    }
}

/// A handle to the Spinner Thread.
///
/// **Important**, be sure to call `close` before dropping this struct
/// to make sure the thread joins before the main thread might close.
/// Otherwise you will get cutoff output.
pub struct SpinnerHandle {
    send: Sender<SpinnerMessage>,
    handle: Option<JoinHandle<()>>,
}

impl SpinnerHandle {
    /// Update the message that is given to the user as part of the spinner
    ///
    /// Returns the String that is put in in case the sender could not send.
    pub fn update(&self, st: String) -> Option<String> {
        match self.send.send(SpinnerMessage::Status(st)) {
            Ok(_) => None,
            Err(s) => {
                if let SendError(SpinnerMessage::Status(st)) = s {
                    Some(st)
                } else {
                    unreachable!()
                }
            }
        }
    }

    /// Print out a message above the Spinner for the user.
    pub fn message(&self, msg: String) -> Option<String> {
        match self.send.send(SpinnerMessage::Message(msg)) {
            Ok(_) => None,
            Err(s) => {
                if let SendError(SpinnerMessage::Message(msg)) = s {
                    Some(msg)
                } else {
                    unreachable!()
                }
            }
        }
    }

    /// Shutdown the thread and wait until it has joined.
    pub fn close(mut self) {
        drop(self.send);
        if let Some(th) = self.handle.take() {
            let _ = th.join();
        }
    }
}

/// The struct with which to create a Spinner, check out the crate documentation
/// for some more details
pub struct SpinnerBuilder {
    msg: String,
    spinner: Option<Vec<&'static str>>,
    custom_format: Option<Box<FormatFn>>,
    step: Option<Duration>,
}

impl SpinnerBuilder {
    /// Create a SpinnerBuilder, giving an original message
    pub fn new(msg: String) -> Self {
        SpinnerBuilder {
            msg,
            spinner: None,
            custom_format: None,
            step: None,
        }
    }

    /// Customize the vector that can be used for the spinner, you can google
    /// something like 'ascii spinner' for some fun examples. Note though that
    /// you should make sure they all have the same length. And only one line.
    pub fn spinner(mut self, sp: Vec<&'static str>) -> Self {
        self.spinner = Some(sp);
        self
    }

    /// Customize the step between each update of the text. The default is 200ms
    pub fn step(mut self, st: Duration) -> Self {
        self.step = Some(st);
        self
    }

    /// Set the format closure that is to be used by the spinner, check out
    /// the complex_spinner example how this could be used.
    pub fn format<F>(mut self, f: F) -> Self
    where
        F: Fn(&str, &str) -> String + Send + 'static,
    {
        self.custom_format = Some(Box::new(f));
        self
    }

    /// Start the thread that takes care of the Spinner and return immediately
    /// allowing you to load or do otherwise operations.
    pub fn start(self) -> SpinnerHandle {
        let ttypes = {
            if let Some(v) = self.spinner {
                v
            } else {
                vec!["▁", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃"]
            }
        };

        let st = {
            if let Some(s) = self.step {
                s
            } else {
                Duration::from_millis(100)
            }
        };

        let (tx, rx) = channel();
        let sp = Spinner {
            status: self.msg,
            types: ttypes,
            custom_out: self.custom_format,
            rx,
            step: st,
        };
        Spinner::start(sp, tx)
    }
}
