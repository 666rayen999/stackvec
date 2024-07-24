# StackVec

**StackVec** is a lightweight vector-like library in Rust that uses stack allocation instead of heap allocation. It provides a familiar interface similar to **Vec**, including operations such as push, pop, rev, sort, iter, and more.

### Features
- Stack-allocated vector to avoid heap allocations
- Supports standard vector operations: push, pop, rev, sort, iter, etc.
- Generic over element type and size
- macro: stackvec!

### Usage
```rust
  let mut vec = StackVec::<u32, 10>::new();

  vec.push(1);
  vec.push(2);
  vec.push(3);

  // Output: [1, 2, 3]

  vec.pop();

  // Output: [1, 2]

  vec.rev();

  // Output: [2, 1]

  vec.sort();

  // Output: [1, 2]

  vec.iter().filter(|x| x > 0).for_each(|x| println("{}", x));

  vec.push(3);
  println!("{:?}", vec.have(3));
  
  // Output: true

  vec.push(3);
  vec.push(2);

  // Output: [1, 2, 3, 3, 2]

  vec.distinct();

  // Output: [1, 2, 3]

  let vec2 = vec.split(1);

  // vec  = [1]
  // vec2 = [2, 3]
```

### Macros
```rust
  let vec = stackvec!(10) // empty vec with capacity 10

  let vec = stackvec!(f32 10) // empty f32 vec with capacity 10

  let vec = stackvec!(6; 3) // [6, 6, 6]

  let vec = stackvec!(10 = 1, 3, 2) // [1, 2, 3] vec with capacity 10
```

### Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### License

This project is licensed under the MIT License.
