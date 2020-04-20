extern crate spinner;

use std::time::Duration;
use std::thread;

use spinner::{SpinnerBuilder, DANCING_KIRBY};

fn main() {
    let sp = SpinnerBuilder::new("Long Running op!".into())
        .spinner(DANCING_KIRBY.to_vec()).step(Duration::from_millis(500)).start();

    thread::sleep(Duration::from_millis(3000));
    sp.message("Updating...".into());
    sp.update("Fixing things...".into());
    thread::sleep(Duration::from_millis(2000));
    sp.message("Done!".into());
    sp.close();
}
