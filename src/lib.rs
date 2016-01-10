//! This library is useful for console applications with long running processes.
//! You create a `Spinner` and then update it however you see fit. Since this
//! happens asynchronously, your user will not be left in the dark about what
//! your app is doing.

use std::sync::mpsc::{Sender, Receiver, channel, SendError, TryRecvError};
use std::io::{Write, stdout};
use std::thread::{self, JoinHandle};
use std::time::Duration;

enum SpinnerMessage {
    Status(String),
    Message(String)
}

type FormatFn = Fn(&str, &str) -> String + Send + 'static;

pub struct Spinner {
    status: String,
    types: Vec<&'static str>,
    rx: Receiver<SpinnerMessage>,
    custom_out: Option<Box<FormatFn>>
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
                thread::sleep(Duration::from_millis(100))
            }
        });

        SpinnerHandle {
            send: tx,
            handle: Some(th),
        }
    }

    pub fn new(st: String) -> SpinnerHandle {
        let (tx, rx) = channel();
        let sp = Spinner {
            status: st,
            types: vec!["▁","▃","▄","▅","▆","▇","█","▇","▆","▅","▄","▃"],
            custom_out: None,
            rx: rx,
        };
        Self::start(sp, tx)
    }

    pub fn new_custom<F>(st: String, f: F) -> SpinnerHandle
        where F: Fn(&str, &str) -> String + Send + 'static
    {
        let (tx, rx) = channel();
        let sp = Spinner {
            status: st,
            types: vec!["▁","▃","▄","▅","▆","▇","█","▇","▆","▅","▄","▃"],
            custom_out: Some(Box::new(f)),
            rx: rx,
        };
        Self::start(sp, tx)
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
