//! Handy patterns to blink

pub mod morse {
    //! Morse Code Patterns
    //!
    //! * Dots are represented by `0b10`.
    //! * Dashes are represented by `0b110`.
    use crate::Pattern;

    pub const DOT: Pattern = Pattern::from_u32(0b10, 2);
    pub const DASH: Pattern = Pattern::from_u32(0b1110, 4);

    pub const A: Pattern = DOT.append(&DASH);
    pub const B: Pattern = DASH.append(&DOT).append(&DOT).append(&DOT);
    pub const C: Pattern = DASH.append(&DOT).append(&DASH).append(&DOT);
    pub const D: Pattern = DASH.append(&DOT).append(&DOT);
    pub const E: Pattern = DOT;
    pub const F: Pattern = DOT.append(&DOT).append(&DASH).append(&DOT);
    pub const G: Pattern = DASH.append(&DASH).append(&DOT);
    pub const H: Pattern = DOT.append(&DOT).append(&DOT).append(&DOT);
    pub const I: Pattern = DOT.append(&DOT);
    pub const J: Pattern = DOT.append(&DASH).append(&DASH).append(&DASH);
    pub const K: Pattern = DASH.append(&DOT).append(&DASH);
    pub const L: Pattern = DOT.append(&DASH).append(&DOT).append(&DOT);
    pub const M: Pattern = DASH.append(&DASH);
    pub const N: Pattern = DASH.append(&DOT);
    pub const O: Pattern = DASH.append(&DASH).append(&DASH);
    pub const P: Pattern = DOT.append(&DASH).append(&DASH).append(&DOT);
    pub const Q: Pattern = DASH.append(&DASH).append(&DOT).append(&DASH);
    pub const R: Pattern = DOT.append(&DASH).append(&DOT);
    pub const S: Pattern = DOT.append(&DOT).append(&DOT);
    pub const T: Pattern = DASH;
    pub const U: Pattern = DOT.append(&DOT).append(&DASH);
    pub const V: Pattern = DOT.append(&DOT).append(&DOT).append(&DASH);
    pub const W: Pattern = DOT.append(&DASH).append(&DASH);
    pub const X: Pattern = DASH.append(&DOT).append(&DOT).append(&DASH);
    pub const Y: Pattern = DASH.append(&DOT).append(&DASH).append(&DASH);
    pub const Z: Pattern = DASH.append(&DASH).append(&DOT).append(&DOT);

    pub const ZERO: Pattern = DASH.append(&DASH).append(&DASH).append(&DASH).append(&DASH);
    pub const ONE: Pattern = DOT.append(&DASH).append(&DASH).append(&DASH).append(&DASH);
    pub const TWO: Pattern = DOT.append(&DOT).append(&DASH).append(&DASH).append(&DASH);
    pub const THREE: Pattern = DOT.append(&DOT).append(&DOT).append(&DASH).append(&DASH);
    pub const FOUR: Pattern = DOT.append(&DOT).append(&DOT).append(&DOT).append(&DASH);
    pub const FIVE: Pattern = DOT.append(&DOT).append(&DOT).append(&DOT).append(&DOT);
    pub const SIX: Pattern = DASH.append(&DOT).append(&DOT).append(&DOT).append(&DOT);
    pub const SEVEN: Pattern = DASH.append(&DASH).append(&DOT).append(&DOT).append(&DOT);
    pub const EIGHT: Pattern = DASH.append(&DASH).append(&DASH).append(&DOT).append(&DOT);
    pub const NINE: Pattern = DASH.append(&DASH).append(&DASH).append(&DASH).append(&DOT);

    pub const FULL_STOP: Pattern = DOT.append(&DASH).append(&DOT).append(&DASH).append(&DOT).append(&DASH);
    pub const COMMA: Pattern = DASH.append(&DASH).append(&DOT).append(&DOT).append(&DASH).append(&DASH);
    pub const COLON: Pattern = DASH.append(&DASH).append(&DASH).append(&DOT).append(&DOT).append(&DOT);
    pub const QUESTION_MARK: Pattern = DOT.append(&DOT).append(&DASH).append(&DASH).append(&DOT).append(&DOT);
    pub const APOSTROPHE: Pattern = DOT.append(&DASH).append(&DASH).append(&DASH).append(&DASH).append(&DOT);
    pub const HYPHEN: Pattern = DASH.append(&DOT).append(&DOT).append(&DOT).append(&DOT).append(&DASH);
    pub const FRACTION_BAR: Pattern = DASH.append(&DOT).append(&DOT).append(&DASH).append(&DOT);
    pub const BRACKETS: Pattern = DASH.append(&DOT).append(&DASH).append(&DASH).append(&DOT).append(&DASH);
    pub const QUOTATION_MARK: Pattern = DOT.append(&DASH).append(&DOT).append(&DOT).append(&DASH).append(&DOT);
    pub const AT_SIGN: Pattern = DOT.append(&DASH).append(&DASH).append(&DOT).append(&DASH).append(&DOT);
    pub const EQUALS_SIGN: Pattern = DASH.append(&DOT).append(&DOT).append(&DOT).append(&DASH);
    pub const ERROR: Pattern = DOT.append(&DOT).append(&DOT).append(&DOT).append(&DOT).append(&DOT).append(&DOT).append(&DOT);

    pub const SOS: Pattern = S.append(&O).append(&S);
}

pub mod blinks {
    //! Common blink patterns

    use crate::Pattern;

    pub const SHORT_ON_OFF: Pattern = Pattern::from_u32(0b10, 2);
    pub const SHORT_OFF_ON: Pattern = SHORT_ON_OFF.reverse();

    pub const MEDIUM_ON_OFF: Pattern = Pattern::from_u32(0b1100, 4);
    pub const MEDIUM_OFF_ON: Pattern = MEDIUM_ON_OFF.reverse();

    pub const LONG_ON_OFF: Pattern = Pattern::from_u32(0b11110000, 8);
    pub const LONG_OFF_ON: Pattern = LONG_ON_OFF.reverse();

    pub const QUARTER_DUTY: Pattern = Pattern::from_u32(0b1000, 4);
}
