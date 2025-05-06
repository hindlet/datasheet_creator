use egui::{Key, KeyboardShortcut, Modifiers};
use egui_keybind::Shortcut;
// use egui_keybind::Shortcut;




const CLOSE_FILE_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::W,
);

const NEXT_FILE_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::CTRL,
    Key::Tab,
);

const PREV_FILE_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers {
        ctrl: true,
        shift: true,
        alt: false,
        mac_cmd: false,
        command: false
    },
    Key::Tab,
);