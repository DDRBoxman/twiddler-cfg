#[rustfmt::skip]
pub enum TwiddlerButtons {
    T1, T2, T3, T4,
    F0L, F0M, F0R,
    F1L, F1M, F1R,
    F2L, F2M, F2R,
    F3L, F3M, F3R,
    F4L, F4M, F4R,
}

pub struct ButtonState {
    pub t1: bool,
    pub t2: bool,
    pub t3: bool,
    pub t4: bool,
    pub f0l: bool,
    pub f0m: bool,
    pub f0r: bool,
    pub f1l: bool,
    pub f1m: bool,
    pub f1r: bool,
    pub f2l: bool,
    pub f2m: bool,
    pub f2r: bool,
    pub f3l: bool,
    pub f3m: bool,
    pub f3r: bool,
    pub f4l: bool,
    pub f4m: bool,
    pub f4r: bool,
}

pub(crate) fn parse_notation(thumb: String, finger: String) -> ButtonState {
    if thumb.contains(&['0', '1', '2', '3', '4'][..]) {
        parse_t4_notation(thumb, finger)
    } else {
        parse_legacy_notation(thumb, finger)
    }
}

fn parse_t4_notation(thumb: String, finger: String) -> ButtonState {
    let mut button_state = ButtonState {
        t1: false,
        t2: false,
        t3: false,
        t4: false,
        f0l: false,
        f0m: false,
        f0r: false,
        f1l: false,
        f1m: false,
        f1r: false,
        f2l: false,
        f2m: false,
        f2r: false,
        f3l: false,
        f3m: false,
        f3r: false,
        f4l: false,
        f4m: false,
        f4r: false,
    };

    // Parse thumb notation
    for button in thumb.chars() {
        match button {
            '1' => button_state.t1 = true,
            '2' => button_state.t2 = true,
            '3' => button_state.t3 = true,
            '4' => button_state.t4 = true,
            _ => (),
        }
    }

    // Parse finger notation
    for finger_button in finger.split_whitespace() {
        let mut chars = finger_button.chars();
        let finger_row = chars.next().unwrap();
        let finger_col = chars.next().unwrap();
        match (finger_row, finger_col) {
            ('0', 'M') => button_state.f0m = true,
            ('1', 'L') => button_state.f1l = true,
            ('1', 'M') => button_state.f1m = true,
            ('1', 'R') => button_state.f1r = true,
            ('2', 'L') => button_state.f2l = true,
            ('2', 'M') => button_state.f2m = true,
            ('2', 'R') => button_state.f2r = true,
            ('3', 'L') => button_state.f3l = true,
            ('3', 'M') => button_state.f3m = true,
            ('3', 'R') => button_state.f3r = true,
            ('4', 'L') => button_state.f4l = true,
            ('4', 'M') => button_state.f4m = true,
            ('4', 'R') => button_state.f4r = true,
            _ => (),
        }
    }

    button_state
}

fn parse_legacy_notation(thumb: String, finger: String) -> ButtonState {
    let mut button_state = ButtonState {
        t1: false,
        t2: false,
        t3: false,
        t4: false,
        f0l: false,
        f0m: false,
        f0r: false,
        f1l: false,
        f1m: false,
        f1r: false,
        f2l: false,
        f2m: false,
        f2r: false,
        f3l: false,
        f3m: false,
        f3r: false,
        f4l: false,
        f4m: false,
        f4r: false,
    };

    // Parse thumb notation
    for button in thumb.chars() {
        match button {
            'N' => button_state.t1 = true,
            'A' => button_state.t2 = true,
            'C' => button_state.t3 = true,
            'S' => button_state.t4 = true,
            _ => (),
        }
    }

    // Parse finger notation
    for (i, finger_button) in finger.chars().enumerate() {
        match finger_button {
            'L' => match i {
                0 => button_state.f1l = true,
                1 => button_state.f2l = true,
                2 => button_state.f3l = true,
                3 => button_state.f4l = true,
                _ => (),
            },
            'O' => match i {
                0 => button_state.f1m = true,
                1 => button_state.f2m = true,
                2 => button_state.f3m = true,
                3 => button_state.f4m = true,
                _ => (),
            },
            'R' => match i {
                0 => button_state.f1r = true,
                1 => button_state.f2r = true,
                2 => button_state.f3r = true,
                3 => button_state.f4r = true,
                _ => (),
            },
            _ => (),
        }
    }

    button_state
}
