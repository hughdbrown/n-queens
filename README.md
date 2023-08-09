# Purpose
Calculate an 8-queen placement on a regular board and a 20-queen placement on a large board.

# Result
Here is the 8-queen result:
```
 .  .  .  Q  .  .  .  .
 .  .  .  .  .  .  Q  .
 .  .  .  .  Q  .  .  .
 .  Q  .  .  .  .  .  .
 .  .  .  .  .  Q  .  .
 Q  .  .  .  .  .  .  .
 .  .  Q  .  .  .  .  .
 .  .  .  .  .  .  .  Q
cargo run --release  0.06s user 0.02s system 10% cpu 0.712 total
```

Timing for different board sizes:
- 10-queen board: 0.05 seconds
- 10-queen board: 0.08 seconds
- 11-queen board: 0.06 seconds
- 12-queen board: 0.30 seconds
- 13-queen board: 0.10 seconds
- 14-queen board: 55 seconds
- 15-queen board: 61 seconds
- 16-queen board: too long
- 17-queen board: too long

# Settings for micro-benchmarks
```
RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo build --release  && time target/release/n-queens
```
