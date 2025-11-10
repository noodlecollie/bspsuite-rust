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

## How do I unwrap an `Option` if it's valid, and return an error if it's not?

Use `ok_or_else()`, followed by `?`.

* `ok_or_else()` converts an `Option<T>` into a `Result<T, Error>`. The argument to `ok_or_else()` is the error that will be part of the result if the option cannot be unwrapped.
* The `?` operator takes the `Result<T, Error>` and returns the error from the function if one was present. If not, it allows the `T` value to be used.

```rust
// This function returns nothing on success, but an error if something goes wrong.
fn myfunc(opt: Option<&str>) -> Result<(), MyError>
{
	// Try and unwrap the string value.
	let value: &str = opt.ok_or_else(MyError::new("Could not unwrap option"))?;

	// Unwrap was successful, so we can now do something with value.
	// ...
}
```

## How do I unwrap a `Result` if it's valid, and return an error if it's not?

Similarly to above, use `or_else()`, followed by `?`.

```rust
// This function returns nothing on success,
// but an error string if something goes wrong.
fn myfunc(res: Result<&str, MyError>) -> Result<(), String>
{
	// Try and unwrap the string value.
	let value: &str =
		res.or_else(|err| Err(format!("An error occurred. {}", err.to_string())))?;

	// Unwrap was successful, so we can now do something with value.
	// ...
}
```
