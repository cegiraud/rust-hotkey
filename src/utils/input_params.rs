use windows::Win32::UI::Input::KeyboardAndMouse::{
    VIRTUAL_KEY, VK_0, VK_6, VK_CONTROL, VK_MENU, VK_OEM_PERIOD, VK_SHIFT, VK_SPACE,
};

pub trait ToInputParams {
    fn pressed(&self) -> InputParams;
    fn released(&self) -> InputParams;
}

impl ToInputParams for VIRTUAL_KEY {
    fn pressed(&self) -> InputParams {
        InputParams {
            vk_code: *self,
            keep: true,
        }
    }

    fn released(&self) -> InputParams {
        InputParams {
            vk_code: *self,
            keep: false,
        }
    }
}

#[derive(Default)]
pub struct InputParams {
    pub vk_code: VIRTUAL_KEY,
    pub keep: bool,
}

impl InputParams {
    pub fn from(c: char) -> Vec<InputParams> {
        match c {
            'a'..='z' => vec![
                VIRTUAL_KEY(c.to_ascii_uppercase() as u16).pressed(),
                VIRTUAL_KEY(c.to_ascii_uppercase() as u16).released(),
            ],
            'A'..='Z' => vec![
                VK_SHIFT.pressed(),
                VIRTUAL_KEY(c.to_ascii_uppercase() as u16).pressed(),
                VIRTUAL_KEY(c.to_ascii_uppercase() as u16).released(),
                VK_SHIFT.released(),
            ],
            '0'..='9' => vec![
                VK_SHIFT.pressed(),
                VIRTUAL_KEY(c as u16).pressed(),
                VIRTUAL_KEY(c as u16).released(),
                VK_SHIFT.released(),
            ],
            '.' => vec![
                VK_SHIFT.pressed(),
                VK_OEM_PERIOD.pressed(),
                VK_OEM_PERIOD.released(),
                VK_SHIFT.released(),
            ],
            '-' => vec![VK_6.pressed(), VK_6.released()],
            '@' => vec![
                VK_CONTROL.pressed(),
                VK_MENU.pressed(),
                VK_0.pressed(),
                VK_0.released(),
                VK_MENU.released(),
                VK_CONTROL.released(),
            ],
            ' ' => vec![VK_SPACE.pressed(), VK_SPACE.released()],
            _ => vec![],
        }
    }
}

pub fn all_control_keys() -> Vec<VIRTUAL_KEY> {
    vec![VK_SHIFT, VK_CONTROL, VK_MENU]
}
