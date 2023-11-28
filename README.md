# TriVec-ta

Inspired by rust-smallvec, it provides three modes of storing data:
- stack
- heap
- disk

Based on the capacities set, TriVec moves its content between the three
stores

## Usage

```rust
// using `trivec` macro
let mut v = trivec![1, 2, 3, 4, 5; 100; 10_000]

let mut v: TriVec<String> = TriVec::new(100, 10_000)
```