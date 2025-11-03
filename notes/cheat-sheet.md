## How do I cast from an enum to an integer?

Make sure the enum derives from `Copy` (and ideally `Clone`), and then use `*var as i32` (or whatever the desired integer type is).

```rust
#[derive(Copy, Clone)]
enum Foo
{
    Bar = 1,
}

// This function takes an enum constant
// and returns its unsigned integer value.
fn f(foo: &Foo) -> u32
{
    *foo as u32
}
```

Reference: https://stackoverflow.com/a/31359044/2054335
