# Conditionals

## `if`

An `if` conditional expression consists of the condition, main block, and optional
`else` block. Every block starts a new scope of visibility.

```rust,no_run,noplaypen
let condition = true;
let c = if condition {
    let a = 5;
    a
} else {
    let b = 10;
    b
};
```

## `match`

The match expression is a syntactic sugar for nested conditional
expressions. Each branch block starts a new scope of visibility.

```rust,no_run,noplaypen
let value = MyEnum::ValueOne;

match value {
    MyEnum::ValueOne => { ... }
    MyEnum::ValueTen => { ... }
    42 => { ... }
    _ => { ... }
}
```

For now, only the following match patterns are supported:
- constant (e.g. `42`)
- path (e.g. `MyEnum::ValueOne`)
- variable binding (e.g. `value`)
- wildcard (`_`)