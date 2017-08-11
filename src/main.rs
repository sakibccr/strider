extern crate libc;
extern crate x11;

use window_system::WindowSystem;

mod window_system;

fn main() {
    let window_system = WindowSystem::new();
    loop {
    }
}



