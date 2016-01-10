//! This library is useful for console applications with long running processes.
//! You create a `Spinner` and then update it however you see fit. Since this
//! happens asynchronously, your user will not be left in the dark about what
//! your app is doing.

mod menu;

pub use menu::Menu;

use std::sync::mpsc::{Sender, Receiver, channel, SendError, TryRecvError};
use std::io::{Write, stdout};
use std::thread::{self, JoinHandle};
use std::time::Duration;

enum SpinnerMessage {
    Status(String),
    Message(String)
}

type FormatFn = Fn(&str, &str) -> String + Send + 'static;

pub static DANCING_KIRBY: [&'static str; 8] = [
"(>'-')>",
"<('-'<)",
"^('-')^",
"<('-'<)",
"(>'-')>",
"<('-'<)",
"^('-')^",
"<('-'<)"
];

pub struct Spinner {
    status: String,
    types: Vec<&'static str>,
    rx: Receiver<SpinnerMessage>,
    custom_out: Option<Box<FormatFn>>,
    step: Duration
}

pub struct SpinnerHandle {
    send: Sender<SpinnerMessage>,
    handle: Option<JoinHandle<()>>,
}

impl Spinner {

    fn start(sp: Spinner, tx: Sender<SpinnerMessage>) -> SpinnerHandle {
        let th = thread::spawn(move|| {
            let mut sp = sp;
            for i in sp.types.iter().cycle() {
                let mut msg = None;
                let mut should_disc = false;
                loop {
                    match sp.rx.try_recv() {
                        Ok(ms) => {
                            match ms {
                                SpinnerMessage::Status(st) => {
                                    sp.status = st
                                },
                                SpinnerMessage::Message(st) => {
                                    msg = Some(st)
                                }
                            }
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

                if should_disc{ break; }

                if let Some(ref cl) = sp.custom_out {
                    print!("\r{}", cl(i, &sp.status[..]));
                } else {
                    print!("\r{} {}", i, sp.status);
                }
                {
                    let x = stdout();
                    x.lock().flush().unwrap();
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

impl SpinnerHandle {
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

    pub fn close(mut self) {
        drop(self.send);
        if let Some(th) = self.handle.take() {
            let _ = th.join();
        }
    }
}

pub struct SpinnerBuilder {
    msg: String,
    spinner: Option<Vec<&'static str>>,
    custom_format: Option<Box<FormatFn>>,
    step: Option<Duration>,
}

impl SpinnerBuilder {
    pub fn new(msg: String) -> Self {
        SpinnerBuilder {
            msg: msg,
            spinner: None,
            custom_format: None,
            step: None,
        }
    }

    pub fn spinner(mut self, sp: Vec<&'static str>) -> Self {
        self.spinner = Some(sp);
        self
    }

    pub fn step(mut self, st: Duration) -> Self {
        self.step = Some(st);
        self
    }

    pub fn format<F>(mut self, f: F) -> Self
        where F: Fn(&str, &str) -> String + Send + 'static
        {
            self.custom_format = Some(Box::new(f));
            self
        }

    pub fn start(self) -> SpinnerHandle {

        let typ = {
            if let Some(v) = self.spinner {
                v
            } else {
                vec!["▁","▃","▄","▅","▆","▇","█","▇","▆","▅","▄","▃"]
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
            types: typ,
            custom_out: self.custom_format,
            rx: rx,
            step: st,
        };
        Spinner::start(sp, tx)
    }
}
