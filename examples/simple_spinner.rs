extern crate spinner;

use std::time::Duration;
use std::thread;

use spinner::Spinner;

fn main() {
    let sp = Spinner::new("Long Running op!".into());
    thread::sleep(Duration::from_millis(2000));
    sp.message("Updating...".into());
    sp.update("Fixing things...".into());
    thread::sleep(Duration::from_millis(2000));
    sp.message("Done!".into());
    sp.close();
}
