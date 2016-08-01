extern crate winapi;
extern crate user32;

use std::ptr::null_mut;
use std::mem::size_of;

use winapi::winnt::LPCSTR;
use winapi::windef::HWND;
use winapi::winuser::INPUT;
use winapi::winuser::INPUT_KEYBOARD;
use winapi::c_int;
use winapi::winuser::LPINPUT;

fn main() {
    //at this point I'm not sure what's safe and what's unsafe
    unsafe {
        //get the handle for the sublime window
        let hwnd_window_result = 
            user32::FindWindowA( null_mut(),
                "untitled - Sublime Text".as_ptr() as LPCSTR)
                as HWND;
        //use handle to bring window to foreground (doesn't work if window is minimized)
        user32::SetForegroundWindow(hwnd_window_result);

        //unconfigured keyboard input
        let mut ip = INPUT {
           type_: INPUT_KEYBOARD,
           u: [0; 4],
        };

        //configure input struct to press the "A" key
        {
            let ki = INPUT::ki_mut(&mut ip);
            ki.wVk = 0x41; // virtual-keycode for the "A" key
            ki.dwFlags = 0; // flag of 0 results in a key press
        }
        //send the input
        user32::SendInput(1, &mut ip as LPINPUT, size_of::<INPUT>() as c_int);
    }
}
