# Blinq - a blinking queue

[![Documentation](https://docs.rs/blinq/badge.svg)](https://docs.rs/blinq)

A queue for toggling a GPIO or blinking an LED, with binary patterns
encoded as a u32.

## Example

```rust
use blinq::{Pattern, Blinq, patterns, consts};

// Create a blink queue with room for 8 patterns, that is active-low
let mut blinq: Blinq<consts::U8, FakeGpio> = Blinq::new(gpio, true);

// Insert "HELLO." in morse code

blinq.enqueue(patterns::morse::H); // 8 steps
blinq.enqueue(patterns::morse::E); // 2 steps
blinq.enqueue(patterns::morse::L); // 10 steps
blinq.enqueue(patterns::morse::L); // 10 steps
blinq.enqueue(patterns::morse::O); // 12 steps
blinq.enqueue(patterns::morse::FULL_STOP); // 18 steps

// This is 60 steps
for _ in 0..60 {
   blinq.step();
}

// The queue is now exhausted, and the GPIO will be driven to the
// inactive state
blinq.step();
```

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
