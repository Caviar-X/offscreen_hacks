use offscreen_hacks::{keyboard,mouse};
use std::io::*;
fn main() {
    loop {
        if keyboard::keyboard_pressed() == None {
            break;
        }
    }
    print!("Offscreen Hacks version 0.1.0 by Caviar-X\n");
    print!("Please enter the width:");
    let mut s = "".into();
    stdout().flush().unwrap();
    stdin().read_line(&mut s).expect("Failed to read");
    let width : u32 = s.trim().parse().expect("Unable to recongize the number!");
    print!("Please enter the key you want to simulate(multiple times please):");
    loop {
        if keyboard::keyboard_pressed() == None {
            break;
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(1));
    let key = keyboard::keyboard_pressed().expect("Cannot find");
    println!("You pressed {}",key);
    println!("Working...");
    loop {
        if mouse::is_in_zone(width as i32) {
            keyboard::keyboard_press(key).expect("Cannot press the key!");
        }
    }
}