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
