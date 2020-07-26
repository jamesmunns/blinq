//! Handy patterns to blink

pub mod morse {
    //! Morse Code Patterns
    //!
    //! * Dots are represented by `0b10`.
    //! * Dashes are represented by `0b110`.
    use crate::Pattern;

    pub const A: Pattern = Pattern::from_u32(0b10110, 5);
    pub const B: Pattern = Pattern::from_u32(0b110101010, 9);
    pub const C: Pattern = Pattern::from_u32(0b1101011010, 10);
    pub const D: Pattern = Pattern::from_u32(0b1101010, 7);
    pub const E: Pattern = Pattern::from_u32(0b10, 2);
    pub const F: Pattern = Pattern::from_u32(0b101011010, 9);
    pub const G: Pattern = Pattern::from_u32(0b11011010, 8);
    pub const H: Pattern = Pattern::from_u32(0b10101010, 8);
    pub const I: Pattern = Pattern::from_u32(0b1010, 4);
    pub const J: Pattern = Pattern::from_u32(0b10110110110, 11);
    pub const K: Pattern = Pattern::from_u32(0b11010110, 8);
    pub const L: Pattern = Pattern::from_u32(0b101101010, 9);
    pub const M: Pattern = Pattern::from_u32(0b110110, 6);
    pub const N: Pattern = Pattern::from_u32(0b11010, 5);
    pub const O: Pattern = Pattern::from_u32(0b110110110, 9);
    pub const P: Pattern = Pattern::from_u32(0b1011011010, 10);
    pub const Q: Pattern = Pattern::from_u32(0b11011010110, 11);
    pub const R: Pattern = Pattern::from_u32(0b1011010, 7);
    pub const S: Pattern = Pattern::from_u32(0b101010, 6);
    pub const T: Pattern = Pattern::from_u32(0b110, 3);
    pub const U: Pattern = Pattern::from_u32(0b1010110, 7);
    pub const V: Pattern = Pattern::from_u32(0b101010110, 9);
    pub const W: Pattern = Pattern::from_u32(0b10110110, 8);
    pub const X: Pattern = Pattern::from_u32(0b1101010110, 10);
    pub const Y: Pattern = Pattern::from_u32(0b11010110110, 11);
    pub const Z: Pattern = Pattern::from_u32(0b1101101010, 10);

    pub const ZERO: Pattern = Pattern::from_u32(0b110110110110110, 15);
    pub const ONE: Pattern = Pattern::from_u32(0b10110110110110, 14);
    pub const TWO: Pattern = Pattern::from_u32(0b1010110110110, 13);
    pub const THREE: Pattern = Pattern::from_u32(0b101010110110, 12);
    pub const FOUR: Pattern = Pattern::from_u32(0b10101010110, 11);
    pub const FIVE: Pattern = Pattern::from_u32(0b1010101010, 10);
    pub const SIX: Pattern = Pattern::from_u32(0b11010101010, 11);
    pub const SEVEN: Pattern = Pattern::from_u32(0b110110101010, 12);
    pub const EIGHT: Pattern = Pattern::from_u32(0b1101101101010, 13);
    pub const NINE: Pattern = Pattern::from_u32(0b11011011011010, 14);

    pub const FULL_STOP: Pattern = Pattern::from_u32(0b101101011010110, 15);
    pub const COMMA: Pattern = Pattern::from_u32(0b1101101010110110, 16);
    pub const COLON: Pattern = Pattern::from_u32(0b110110110101010, 15);
    pub const QUESTION_MARK: Pattern = Pattern::from_u32(0b10101101101010, 14);
    pub const APOSTROPHE: Pattern = Pattern::from_u32(0b1011011011011010, 16);
    pub const HYPHEN: Pattern = Pattern::from_u32(0b11010101010110, 14);
    pub const FRACTION_BAR: Pattern = Pattern::from_u32(0b110101011010, 12);
    pub const BRACKETS: Pattern = Pattern::from_u32(0b1101011011010110, 16);
    pub const QUOTATION_MARK: Pattern = Pattern::from_u32(0b10110101011010, 14);
    pub const AT_SIGN: Pattern = Pattern::from_u32(0b101101101011010, 15);
    pub const EQUALS_SIGN: Pattern = Pattern::from_u32(0b110101010110, 12);
    pub const ERROR: Pattern = Pattern::from_u32(0b1010101010101010, 16);

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
