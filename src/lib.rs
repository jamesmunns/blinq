//! blinq - a blinking queue
//!
//! A queue for toggling a GPIO or blinking an LED, with binary patterns
//! encoded as a u32.
//!
//! ## Example
//!
//! ```rust
//! # use core::sync::atomic::{AtomicBool, Ordering};
//! # use embedded_hal::digital::v2::OutputPin;
//! #
//! # struct FakeGpio {
//! #     state: &'static AtomicBool,
//! # }
//! #
//! # impl OutputPin for FakeGpio {
//! #     type Error = ();
//! #     fn set_low(&mut self) -> Result<(), ()> {
//! #         self.state.store(false, Ordering::SeqCst);
//! #         Ok(())
//! #     }
//! #     fn set_high(&mut self) -> Result<(), ()> {
//! #         self.state.store(true, Ordering::SeqCst);
//! #         Ok(())
//! #     }
//! # }
//! #
//! # static STATE: AtomicBool = AtomicBool::new(false);
//! # let gpio = FakeGpio { state: &STATE };
//! #
//! use blinq::{Pattern, Blinq, patterns, consts};
//!
//! // Create a blink queue with room for 8 patterns, that is active-low
//! let mut blinq: Blinq<consts::U8, FakeGpio> = Blinq::new(gpio, true);
//!
//! // Insert "HELLO." in morse code
//!
//! blinq.enqueue(patterns::morse::H); // 8 steps
//! blinq.enqueue(patterns::morse::E); // 2 steps
//! blinq.enqueue(patterns::morse::L); // 9 steps
//! blinq.enqueue(patterns::morse::L); // 9 steps
//! blinq.enqueue(patterns::morse::O); // 9 steps
//! blinq.enqueue(patterns::morse::FULL_STOP); // 15 steps
//!
//! // This is 52 steps
//! for _ in 0..52 {
//!    blinq.step();
//! }
//!
//! // The queue is now exhausted, and the GPIO will be driven to the
//! // inactive state
//! blinq.step();
//! ```

#![cfg_attr(not(test), no_std)]

use embedded_hal::digital::v2::OutputPin;

use heapless::{
    spsc::{Queue, SingleCore},
    ArrayLength,
};

pub use heapless::consts;

pub mod patterns;

/// A blinking pattern encoded as a u32
///
/// These patterns are used with a Blinq
#[derive(Clone)]
pub struct Pattern {
    pattern: u32,
    used: u8,
}

impl Pattern {
    /// Create a new pattern from a u32
    ///
    /// Note: if `used` is larger than 32, it will cause a compile time error.
    /// In the future this will silently truncate to 32.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use blinq::Pattern;
    ///
    /// // This is a blink with a 25% on, 75% off duty cycle
    /// let on_off = Pattern::from_u32(0b1000, 4);
    /// ```
    pub const fn from_u32(pattern: u32, used: u8) -> Self {
        let pat = pattern.reverse_bits();
        let pat = pat >> (32 - used);
        Pattern { pattern: pat, used }
    }

    /// Create new pattern by appending one to the other
    ///
    /// Note: If self.used + other.used is greater than 32, the
    /// pattern will be truncated. If self.used is greater than
    /// 32, this will fail to compile. In the future, this will
    /// silently truncate to 32.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use blinq::Pattern;
    ///
    /// // This is on for a single step
    /// let on = Pattern::from_u32(0b1, 1);
    ///
    /// // This is off for three steps
    /// let off = Pattern::from_u32(0b000, 3);
    ///
    /// // This is on for a single step, then off for 3
    /// let on_off = on.append(&off);
    /// ```
    pub const fn append(&self, other: &Pattern) -> Pattern {
        let pat = self.pattern | other.pattern << self.used;
        Pattern {
            pattern: pat,
            used: self.used + other.used,
        }
    }

    /// Reverse the bit pattern, keeping the same length
    /// ## Example
    ///
    /// ```rust
    /// use blinq::Pattern;
    ///
    /// // This is a blink with a 25% on, 75% off duty cycle
    /// let on_off = Pattern::from_u32(0b1000, 4);
    ///
    /// // This is a blink with a 75% off, 25% on duty cycle
    /// let off_on = on_off.reverse();
    /// ```
    pub const fn reverse(&self) -> Pattern {
        Pattern::from_u32(self.pattern, self.used)
    }

    /// Step one bit in the pattern
    fn step(&mut self) -> bool {
        let result = self.pattern & 0b1 == 1;
        self.pattern = self.pattern.rotate_right(1);
        result
    }
}

/// A blinking queue
///
/// This takes an embedded-hal OutputPin, and drives it based on
/// given patterns on each step.
///
/// ## Example
///
/// ```rust
/// # use core::sync::atomic::{AtomicBool, Ordering};
/// # use embedded_hal::digital::v2::OutputPin;
/// #
/// # struct FakeGpio {
/// #     state: &'static AtomicBool,
/// # }
/// #
/// # impl OutputPin for FakeGpio {
/// #     type Error = ();
/// #     fn set_low(&mut self) -> Result<(), ()> {
/// #         self.state.store(false, Ordering::SeqCst);
/// #         Ok(())
/// #     }
/// #     fn set_high(&mut self) -> Result<(), ()> {
/// #         self.state.store(true, Ordering::SeqCst);
/// #         Ok(())
/// #     }
/// # }
/// #
/// # static STATE: AtomicBool = AtomicBool::new(false);
/// # let gpio = FakeGpio { state: &STATE };
/// #
/// use blinq::{Pattern, Blinq, patterns, consts};
///
/// // Create a blink queue with room for 8 patterns, that is active-low
/// let mut blinq: Blinq<consts::U8, FakeGpio> = Blinq::new(gpio, true);
///
/// // Insert "HELLO." in morse code
///
/// blinq.enqueue(patterns::morse::H); // 8 steps
/// blinq.enqueue(patterns::morse::E); // 2 steps
/// blinq.enqueue(patterns::morse::L); // 9 steps
/// blinq.enqueue(patterns::morse::L); // 9 steps
/// blinq.enqueue(patterns::morse::O); // 9 steps
/// blinq.enqueue(patterns::morse::FULL_STOP); // 15 steps
///
/// // This is 52 steps
/// for _ in 0..52 {
///    blinq.step();
/// }
///
/// // The queue is now exhausted, and the GPIO will be driven to the
/// // inactive state
/// blinq.step();
/// ```
pub struct Blinq<N, G>
where
    N: ArrayLength<Pattern>,
    G: OutputPin,
{
    current: Option<Pattern>,
    queue: Queue<Pattern, N, u8, SingleCore>,
    step: u8,
    gpio: G,
    active_low: bool,
}

impl<N, G> Blinq<N, G>
where
    N: ArrayLength<Pattern>,
    G: OutputPin,
{
    /// Create a new Blinq with the given GPIO
    ///
    /// The GPIO will be driven to the "inactive" state
    /// on creation
    pub fn new(mut gpio: G, active_low: bool) -> Self {
        if active_low {
            gpio.set_high().ok();
        } else {
            gpio.set_low().ok();
        }

        Self {
            current: None,
            queue: unsafe { Queue::u8_sc() },
            step: 0,
            gpio,
            active_low,
        }
    }

    /// Enqueue a new pattern into the queue
    ///
    /// If the queue is currently full, the pattern will be discarded
    pub fn enqueue(&mut self, pat: Pattern) {
        self.queue.enqueue(pat).ok();
    }

    /// Try to enqueue a new pattern into the queue
    ///
    /// If the queue is currently full, an error will be returned
    pub fn try_enqueue(&mut self, pat: Pattern) -> Result<(), Pattern> {
        self.queue.enqueue(pat)
    }

    /// Move the queue one step
    ///
    /// This will update the GPIO with the next state in the current
    /// pattern, or start the next pattern. If the queue is empty,
    /// the GPIO will be driven to the inactive state.
    ///
    /// If any GPIO errors occur, they will be discarded, but the
    /// pattern will still step forward.
    ///
    /// blinq has no concept of time, so you should call it at a rate
    /// that makes sense for you. For example, if you wanted the pattern
    /// `0b101010` to be a 1hz blink, you should call `step` every 500ms.
    /// If you want `0b11110000` to be a 1hz blink, you should call `step`
    /// every 125ms.
    pub fn step(&mut self) {
        let _ = self.try_step();
    }

    /// Try to move the queue one step
    ///
    /// This will update the GPIO with the next state in the current
    /// pattern, or start the next pattern. If the queue is empty,
    /// the GPIO will be driven to the inactive state.
    ///
    /// If any GPIO errors occur, they will be returned, but the
    /// pattern will still step forward.
    ///
    /// blinq has no concept of time, so you should call it at a rate
    /// that makes sense for you. For example, if you wanted the pattern
    /// `0b101010` to be a 1hz blink, you should call `step` every 500ms.
    /// If you want `0b11110000` to be a 1hz blink, you should call `step`
    /// every 125ms.
    pub fn try_step(&mut self) -> Result<(), G::Error> {
        // Attempt to load a pattern if none is currently active
        if self.current.is_none() {
            while let Some(pat) = self.queue.dequeue() {
                // Only take non-empty patterns
                if pat.used != 0 {
                    self.current = Some(pat);
                    break;
                }
            }
        }

        let state = match self.current.take() {
            None => {
                // No pattern, drive GPIO inactive
                false
            }
            Some(mut pat) => {
                // TODO: remove this after Rust >= 1.46 where we can truncate lens
                // in const-fns
                pat.used = pat.used.min(32);

                // Walk step and counter
                let state = pat.step();
                self.step += 1;

                // If we have exhausted this pattern, reset our step counter to zero.
                // Otherwise, return the pattern to current.
                if self.step >= pat.used {
                    self.step = 0;
                } else {
                    self.current = Some(pat);
                }

                state
            }
        };

        // Drive the GPIO. This should be last, in case errors occur
        if state ^ self.active_low {
            self.gpio.set_high()?;
        } else {
            self.gpio.set_low()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::morse::SOS;
    use heapless::consts::*;

    use core::sync::atomic::{AtomicBool, Ordering};

    struct FakeGpio {
        state: &'static AtomicBool,
    }

    impl OutputPin for FakeGpio {
        type Error = ();
        fn set_low(&mut self) -> Result<(), ()> {
            self.state.store(false, Ordering::SeqCst);
            Ok(())
        }
        fn set_high(&mut self) -> Result<(), ()> {
            self.state.store(true, Ordering::SeqCst);
            Ok(())
        }
    }

    #[test]
    fn simple() {
        static STATE: AtomicBool = AtomicBool::new(false);
        let fg = FakeGpio { state: &STATE };
        let mut stepr: Blinq<U1, FakeGpio> = Blinq::new(fg, false);
        stepr.enqueue(SOS);

        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), true);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), false);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), true);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), false);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), true);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), false);

        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), true);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), true);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), false);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), true);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), true);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), false);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), true);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), true);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), false);

        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), true);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), false);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), true);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), false);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), true);
        stepr.step();
        assert_eq!(STATE.load(Ordering::SeqCst), false);
    }

    #[test]
    fn queued() {
        static STATE: AtomicBool = AtomicBool::new(false);
        let fg = FakeGpio { state: &STATE };
        let mut stepr: Blinq<U3, FakeGpio> = Blinq::new(fg, false);
        stepr.enqueue(SOS);
        stepr.enqueue(SOS);
        stepr.enqueue(SOS);

        for _ in 0..3 {
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), true);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), false);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), true);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), false);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), true);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), false);

            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), true);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), true);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), false);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), true);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), true);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), false);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), true);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), true);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), false);

            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), true);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), false);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), true);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), false);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), true);
            stepr.step();
            assert_eq!(STATE.load(Ordering::SeqCst), false);
        }
    }
}
