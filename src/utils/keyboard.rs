use crate::utils::input_params::{all_control_keys, InputParams};
use std::mem::size_of;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
};

use super::input_params::ToInputParams;

pub fn send_string_event(s: &str) {
    s.chars()
        .flat_map(InputParams::from)
        .for_each(|vkey| send_input(vkey));
}

pub fn release_all_key() {
    all_control_keys()
        .iter()
        .for_each(|vkey| send_input(vkey.released()));
}

fn send_input(param: InputParams) {
    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: param.vk_code,
                wScan: 0,
                dwFlags: if param.keep {
                    KEYBD_EVENT_FLAGS(0)
                } else {
                    KEYEVENTF_KEYUP
                },
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        SendInput(&[input], size_of::<INPUT>() as i32);
    }
}
