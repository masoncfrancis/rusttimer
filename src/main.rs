use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use crossterm::{
    execute,
};

fn main() {
    // Hide the cursor
    match execute!(io::stdout(), crossterm::cursor::Hide) {
        Ok(_) => (),
        Err(e) => eprintln!("Error hiding cursor: {}", e),
    }

    let seconds: i32 = 5;
    let minutes: i32 = 1;
    let hours: i32 = 1;

    // Run timer
    sec_timer(5);
    min_timer(minutes, seconds);
    hr_timer(hours, minutes, seconds);
}

fn sec_timer(seconds: i32) {
    if seconds == 0 {
        print!("{}", "\r".to_owned() + "Time's up!");
        return;
    } else {
        print!("{}", "\r".to_owned() + &seconds.to_string());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        sec_timer(seconds - 1);
    }
}

fn min_timer(minutes: i32, seconds: i32) {
    if seconds == 0 && minutes == 0 {
        print!("{}", "\r".to_owned() + "Time's up!");
        return;
    } else {
        // Finish this
    }
}

fn hr_timer(hours: i32, minutes: i32, seconds: i32) {
    if hours == 0 && seconds == 0 && minutes == 0 {
        print!("{}", "\r".to_owned() + "Time's up!");
        return;
    } else {
        // Finish this
    }
}
