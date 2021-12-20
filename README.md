


# No-Await
**Disclaimer: this is a proof of concept. It is not intended to be used in production. **

This crate provides a proc-macro that removes the need the use the `await` keyword.
```rust
#[suspend]
fn foo() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.rust-lang.org")?;
    let text = response.text()?; 
    println!("{}", text);
    Ok(())
}
```
the above code is functionally equivalent to:
```rust

async fn foo() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.rust-lang.org").await?;
    println!("{}", response.text().await?);
    Ok(())
}
```
The crate is presented as a proposal for the allowing the following syntax: 
```rust
suspend fn foo() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.rust-lang.org")?;
    println!("{}", response.text()?);
    Ok(())
}
```