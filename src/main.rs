mod utils;

use std::env;

use utils::keyboard::{release_all_key, send_string_event};

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, MOD_CONTROL, MOD_SHIFT, VK_A};
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, TranslateMessage, MSG, WM_HOTKEY,
};

const HOTKEY_ID: i32 = 1;

fn main() {
    // Récupère les arguments de la ligne de commande
    let args: Vec<String> = env::args().collect();

    // Vérifie qu'au moins un argument a été passé
    if args.len() < 2 {
        eprintln!("Usage: {} <string>", args[0]);
        std::process::exit(1);
    }

    // Concatène tous les arguments en une seule chaîne
    let input_string = args[1..].join(" ");

    unsafe {
        let result = RegisterHotKey(
            HWND(std::ptr::null_mut()),
            HOTKEY_ID,
            MOD_CONTROL | MOD_SHIFT,
            VK_A.0 as u32,
        );
        if result.is_err() {
            eprintln!("Impossible d'enregistrer le raccourci clavier");
            return;
        }

        // Boucle pour recevoir les messages de raccourcis clavier
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, HWND(std::ptr::null_mut()), 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);

            if msg.message == WM_HOTKEY && msg.wParam.0 == HOTKEY_ID as usize {
                println!("Raccourci clavier !");
                release_all_key();
                send_string_event(&input_string);
            }
        }
    }
}
