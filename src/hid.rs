use bimap::BiMap;
use std::collections::HashMap;
use std::sync::OnceLock;

fn modifiers_hid() -> &'static HashMap<&'static str, usize> {
    static MODIFIERS: OnceLock<HashMap<&str, usize>> = OnceLock::new();
    MODIFIERS.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("L-Ctrl", 0x1);
        m.insert("L-Shift", 0x2);
        m.insert("L-Alt", 0x4);
        m.insert("L-GUI", 0x8);
        m.insert("R-Ctrl", 0x10);
        m.insert("R-Shift", 0x20);
        m.insert("R-Alt", 0x40);
        m.insert("R-GUI", 0x80);
        m
    })
}

pub fn keys_hid() -> &'static BiMap<u8, String> {
    static KEYS: OnceLock<BiMap<u8, String>> = OnceLock::new();
    KEYS.get_or_init(|| {
        let mut elements: bimap::BiMap<u8, String> = BiMap::new();

        // Alpha keys
        for i in 0x04..=0x1D {
            elements.insert(i, format!("{}", (i - 0x04 + b'a') as char));
        }

        // numbers
        elements.insert(0x1E, "1".to_owned());
        elements.insert(0x1F, "2".to_owned());
        elements.insert(0x20, "3".to_owned());
        elements.insert(0x21, "4".to_owned());
        elements.insert(0x22, "5".to_owned());
        elements.insert(0x23, "6".to_owned());
        elements.insert(0x24, "7".to_owned());
        elements.insert(0x25, "8".to_owned());
        elements.insert(0x26, "9".to_owned());
        elements.insert(0x27, "0".to_owned());

        // F keys
        for i in 0x3A..=0x45 {
            elements.insert(i, format!("F{}", i - 0x3A + 1));
        }

        // F keys 2
        for i in 0x68..=0x73 {
            elements.insert(i, format!("F{}", i - 0x68 + 1));
        }

        // keypad
        elements.insert(0x54, "KP/".to_owned());
        elements.insert(0x55, "KP*".to_owned());
        elements.insert(0x56, "KP-".to_owned());
        elements.insert(0x57, "KP+".to_owned());
        elements.insert(0x58, "KPEnter".to_owned());
        elements.insert(0x59, "KP1".to_owned());
        elements.insert(0x5A, "KP2".to_owned());
        elements.insert(0x5B, "KP3".to_owned());
        elements.insert(0x5C, "KP4".to_owned());
        elements.insert(0x5D, "KP5".to_owned());
        elements.insert(0x5E, "KP6".to_owned());
        elements.insert(0x5F, "KP7".to_owned());
        elements.insert(0x60, "KP8".to_owned());
        elements.insert(0x61, "KP9".to_owned());
        elements.insert(0x62, "KP0".to_owned());
        elements.insert(0x63, "KP.".to_owned());
        elements.insert(0x64, "KP=".to_owned());

        // others
        elements.insert(0x28, "Return".to_owned());
        elements.insert(0x29, "Escape".to_owned());
        elements.insert(0x2A, "Backspace".to_owned());
        elements.insert(0x2B, "Tab".to_owned());
        elements.insert(0x39, "CapsLock".to_owned());
        elements.insert(0x46, "PrintScreen".to_owned());
        elements.insert(0x47, "ScrollLock".to_owned());
        elements.insert(0x48, "Pause".to_owned());
        elements.insert(0x49, "Insert".to_owned());
        elements.insert(0x4A, "Home".to_owned());
        elements.insert(0x4B, "PageUp".to_owned());
        elements.insert(0x4C, "Delete".to_owned());
        elements.insert(0x4D, "End".to_owned());
        elements.insert(0x4E, "PageDown".to_owned());
        elements.insert(0x4F, "RightArrow".to_owned());
        elements.insert(0x50, "LeftArrow".to_owned());
        elements.insert(0x51, "DownArrow".to_owned());
        elements.insert(0x52, "UpArrow".to_owned());
        elements.insert(0x53, "NumLock".to_owned());
        elements.insert(0x65, "Application".to_owned());

        elements
    })
}
