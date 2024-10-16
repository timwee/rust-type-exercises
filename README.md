# Source

- [https://github.com/skyzh/type-exercise-in-rust](Type Exercise in Rust)

# Array and ArrayBuilder

## Motivation

- Give developers ability to create generic functions over all types of arrays -- no matter if it's a fixed length type like `i32` in `Int32Array`, `f32` in `Float32Array`, or variable length type like `String` in `StringArray`.

ie:

```rust
fn build_array_from_vec<A: Array>(items: &[Option<A::RefItem<'_>>]) -> A {
    let mut builder = A::Builder::with_capacity(items.len());
    for item in items {
        builder.push(*item);
    }
    builder.finish()
}

#[test]
fn test_build_int32_array() {
    let data = vec![Some(1), Some(2), Some(3), None, Some(5)];
    let array = build_array_from_vec::<I32Array>(&data[..]);
}

#[test]
fn test_build_string_array() {
    let data = vec![Some("1"), Some("2"), Some("3"), None, Some("5"), Some("")];
    let array = build_array_from_vec::<StringArray>(&data[..]);
}
```

# Scalar and ScalarRef

## Motivation

- Add ability to write more generic functions with zero runtime overhead on type matching and conversion, especially between owned and referenced types.
- Tie this with `Array` and `ArrayBuilder` so that we can write functions more easily for these types.

## Example

Without our Scalar implement, there could be problems:

```rust
fn build_array_repeated_owned<A: Array>(item: A::OwnedItem, len: usize) -> A {
    let mut builder = A::Builder::with_capacity(len);
    for _ in 0..len {
        builder.push(Some(item /* How to convert `item` to `RefItem`? */));
    }
    builder.finish()
}
```

With Scalar trait and corresponding implements,

```rust
fn build_array_repeated_owned<A: Array>(item: A::OwnedItem, len: usize) -> A {
    let mut builder = A::Builder::with_capacity(len);
    for _ in 0..len {
        builder.push(Some(item.as_scalar_ref())); // Now we have `as_scalar_ref` on `Scalar`!
    }
    builder.finish()
}
```

# `ArrayImpl`, `ArrayBuilderImpl`, `ScalarImpl` and `ScalarRefImpl`

## Motivation

- Implement `TryFrom` and `From` for `ArrayImpl` and `ArrayBuilderImpl` so that we can convert between owned and referenced types easily.

# BinaryExpression

## Motivation

Allow users to write function such as

```rust
pub fn str_contains(i1: &str, i2: &str) -> bool {
    i1.contains(i2)
}
```

And have it work for `StringArray` and `&str`

```rust
fn test_str_contains() {
    let expr = BinaryExpression::<StringArray, StringArray, BoolArray, _>::new(str_contains);
    let result = expr
        .eval(
            &StringArray::from_slice(&[Some("000"), Some("111"), None]).into(),
            &StringArray::from_slice(&[Some("0"), Some("0"), None]).into(),
        )
        .unwrap();
    check_array_eq::<BoolArray>(
        (&result).try_into().unwrap(),
        &[Some(true), Some(false), None],
    );
}
```
