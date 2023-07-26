# MAHF COCO

[MAHF](https://github.com/mahf-opt/mahf) bindings for the [COCO](https://github.com/numbbo/coco) benchmarking framework.

## Getting Started

Add the following to your `Cargo.toml`:

```toml
[dependencies]
mahf = "0.1.0"
mahf_coco = "0.1.0"
```

Constructing the `bbob` suite and iterating through the problem instances:

```rust
use mahf_coco::{Suite, SuiteName};

let mut suite = Suite::new(SuiteName::Bbob);

for problem in suite {
    /* ... */
}
```

# License

This project is licensed under
the [GNU General Public License v3.0](https://github.com/mahf-opt/mahf/blob/master/LICENSE).
