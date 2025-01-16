## How to run the demo
```shell
pnpm install
pnpm run build
npx swc ./index.js
```
## Current behavior

- When the transform is successful, warnings are not printed.
- When there is an error during the transform, warnings are printed.


## Expected behavior

When `npx swc index.js` succeeds, warnings should still be printed in the terminal.


## How to Reproduce the Issue
Here is my code, and no warnings are printed.
```rust
"createSelectorQuery" => {
    HANDLER.with(|handler| {
        handler
            .struct_span_warn(
                n.span,
                format!(
                    "WARNING: {} is deprecated.",
                    id.sym.to_string()
                )
                .as_str(),
            )
            .emit()
    });
}
"getElementById" => {
    HANDLER.with(|handler| {
        handler
            .struct_span_warn(
                n.span,
                format!(
                    "WARNING: {} is deprecated.",
                    id.sym.to_string()
                )
                .as_str(),
            )
            .emit()
    });
}
```

If I change one of the `struct_span_warn` calls to `struct_span_err`, both warnings and errors will be printed on the terminal.

```rust
"createSelectorQuery" => {
    HANDLER.with(|handler| {
        handler
            .struct_span_error(
                n.span,
                format!(
                    "WARNING: {} is deprecated.",
                    id.sym.to_string()
                )
                .as_str(),
            )
            .emit()
    });
}
"getElementById" => {
    HANDLER.with(|handler| {
        handler
            .struct_span_warn(
                n.span,
                format!(
                    "WARNING: {} is deprecated.",
                    id.sym.to_string()
                )
                .as_str(),
            )
            .emit()
    });
}
```


