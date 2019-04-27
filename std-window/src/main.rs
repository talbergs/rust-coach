mod window;
use window::Window;

use regex::Regex;
use std::io;

fn main() {
    let h = 12;
    let mut win = Window::new(h);

    let rex: String = match std::env::args().skip(1).next() {
        Some(v) => v,
        None => String::from("^#\\{\\{(.*)\\}\\}"),
    };

    let stdin_handle = io::stdin();

    loop {
        // std::thread::sleep(std::time::Duration::from_millis(100));
        let mut buf = String::new();

        if let Ok(len) = stdin_handle.read_line(&mut buf) {
            if len == 0 {
                break;
            }
            match Regex::new(&rex) {
                Ok(reg) => {
                    if reg.is_match(&buf) {
                        let s = reg.captures(&buf).unwrap().get(1).unwrap().as_str();
                        win.set_title(s.to_string());
                    }
                }
                Err(_) => {}
            }
            win.push_line(buf);
            win.present();
        }
    }
}
