use egui::{Key, KeyboardShortcut, Modifiers};




pub const CLOSE_FILE_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::W,
);

pub const SAVE_FILE_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers::COMMAND,
    Key::S
);

// Currently shortcuts with tab do not work

// pub const NEXT_FILE_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(
//     Modifiers::CTRL,
//     Key::Tab,
// );

// pub const PREV_FILE_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(
//     Modifiers {
//         ctrl: true,
//         shift: true,
//         alt: false,
//         mac_cmd: false,
//         command: false
//     },
//     Key::Tab,
// );