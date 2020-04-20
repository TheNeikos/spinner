extern crate spinner;

use std::thread;
use std::time::Duration;

use spinner::SpinnerBuilder;

fn main() {
    let sp = SpinnerBuilder::new("Long Running op!".into())
        .format(|sp, status| {
            format!(
                "{spin} -- Currently working on: \'{status}\' -- {spin}",
                spin = sp,
                status = status
            )
        })
        .start();
    thread::sleep(Duration::from_millis(2000));
    sp.message("Updating...".into());
    sp.update("Fixing things...".into());
    thread::sleep(Duration::from_millis(2000));
    sp.message("Done!".into());
    sp.close();
}
