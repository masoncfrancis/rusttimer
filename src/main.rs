use std::thread;
use std::time::Duration;
use std::io::{self, Write};

fn main() {
    run_timer(5);
}

fn run_timer(seconds: i32) {

    if seconds == 0 {
        print!("{}", "\r".to_owned() + "Time's up!");
        return;
    } else {
        print!("{}", "\r".to_owned() + &seconds.to_string());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        run_timer(seconds - 1);
    }
}
