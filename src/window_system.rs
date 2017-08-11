use std::ptr;
use x11::xlib::{ Display, Window };
use x11::xlib::{ XOpenDisplay, XDefaultScreenOfDisplay, XRootWindowOfScreen };

pub struct WindowSystem {
    display: *mut Display,
    root:    Window
}


impl WindowSystem {
    pub fn new() -> WindowSystem {
        unsafe {
            let display = XOpenDisplay(ptr::null_mut());
            let screen  = XDefaultScreenOfDisplay(display);
            let root    = XRootWindowOfScreen(screen);

            WindowSystem {
                display: display,
                root:    root
            }
        }
    }
}


