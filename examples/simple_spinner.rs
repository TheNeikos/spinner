extern crate spinner;

use std::time::Duration;
use std::thread;

use spinner::SpinnerBuilder;

fn main() {
    let sp = SpinnerBuilder::new("Long Running op!".into()).start();
    thread::sleep(Duration::from_millis(2000));
    sp.message("Updating...".into());
    sp.update("Fixing things...".into());
    thread::sleep(Duration::from_millis(2000));
    sp.done("Done!".into());
    sp.close();
}
