


# No-Await
**Disclaimer: this was mostly made as a proof of concept. 
I have tested if there is a performance cost to this macro. **

This crate provides a proc-macro that removes the need the use the `await` keyword.

For example: 
```rust
#[suspend_fn]
fn visit_rustlang() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.rust-lang.org")?;
    let text = response.text()?;
    println!("{}", text);
    text.len(); // sync functions work just fine!
    Ok(())
}
```
the above code is functionally equivalent to:
```rust
async fn visit_rustlang() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.rust-lang.org").await?;
    let text = response.text().await?;
    println!("{}", text);
    
    text.len(); 
    Ok(())
}
```

## Scaping futures
In simple terms, the macro works by inserting a `.await` after every future. 

if you need to escape a future, you can use an async block:
```
```

## Limitations
Currently, the macro does not work inside other macros. 
For example: 
```rust
println!("{}", async_fn());
```
the above code will raise the following error message: 
```rust
| println!("{}", async_fn()); 
|                ^^^^^^^^^^  `impl Future` cannot be formatted with the default formatter
```


The crate is presented as a proposal for allowing the following syntax: 
```rust
suspend fn foo() -> Result<(), reqwest::Error> {
   /* implicit await */
}
```