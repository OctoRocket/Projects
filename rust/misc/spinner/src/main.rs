use anyhow::Result;
use std::time::Duration;
use std::thread::sleep;
use spinner::Spinner;

fn main() -> Result<()> {
    print!("Look at my spinner! >> ");
    let mut spinner = Spinner::new(Duration::from_millis(100));
    spinner.start();
    sleep(Duration::from_secs(5));
    spinner.stop()?;
    println!("Wow!!");

    Ok(())
}
