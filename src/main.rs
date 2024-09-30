use colored::Colorize;
use std::io::{stdout, Write};
use std::process::exit;
use uiohook_rs::{EventHandler, KeyboardEvent, KeyboardEventType, Uiohook, UiohookEvent};

struct SimpleEventHandler;

impl SimpleEventHandler {
    fn handle_keyboard_event(&self, event: &KeyboardEvent) {
        match event.event_type {
            KeyboardEventType::Pressed => {
                let open = "[".truecolor(104, 123, 143);
                let on = "on".bold();
                let close = "]".truecolor(104, 123, 143);
                let key = format!("{:?}", event.key_code).bold();

                print!("\n{open}{on} {close} {key}");
                stdout().flush().unwrap();
            }
            KeyboardEventType::Released => {
                let open = "[".truecolor(104, 123, 143);
                let off = "off".truecolor(114, 114, 114);
                let close = "]".truecolor(104, 123, 143);
                let key = format!("{:?}", event.key_code).bold();

                print!("\n{open}{off}{close} {key}");
                stdout().flush().unwrap();
            }
            _ => {}
        }
    }
}

impl EventHandler for SimpleEventHandler {
    fn handle_event(&self, event: &UiohookEvent) {
        match event {
            UiohookEvent::Keyboard(e) => self.handle_keyboard_event(e),
            _ => {}
        }
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let title = "simple-key-monitor".truecolor(88, 157, 246).bold();
    let version = format!("v{}", VERSION).normal();

    print!("{} {}", title, version);
    stdout().flush().unwrap();

    let event_handler = SimpleEventHandler;
    let uiohook = Uiohook::new(event_handler);

    if let Err(e) = uiohook.run() {
        eprintln!("Failed to run uiohook: {}", e);
        exit(1);
    }

    loop {}
}
