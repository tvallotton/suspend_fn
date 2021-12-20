# suspend fn
**Disclaimer: this was mostly made as a proof of concept for the proposal below. 
I haven't tested if there is a performance cost to this macro.**

This crate provides a proc-macro that removes the need for the `await` keyword. 
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

## suspend blocks
You can also use the `suspend!` and `suspend_move!` macros similarlly to `async` and `async move` blocks.
Note that these might prompt style warnings, so I recommend placing `#![allow(unused_parens)]` in the crate root.
```rust
#![allow(unused_parens)]
suspend! { 
    let response = reqwest::get("https://www.rust-lang.org")?;
    let text = response.text()?;
    println!("{}", text);
    text.len(); 
}
```

## limitations
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

for such cases you may simply use `.await` in the macro yourself. 

## motivation
In a [recent blog post](https://carllerche.com/2021/06/17/six-ways-to-make-async-rust-easier/) discussing the async cacellation problem, it was argued that 
async destructors would lead to inconsistencies from a langauge design perspective. 
This is because the compiler would introduce `.await` calls for async destructors, 
making some `.await` calls implicit and others explicit. 

As a way to resolve this conflict, it was proposed the removal of the `.await` syntax entirely. 
Although I think this would me great from an ergonomics perspective, I would not like 
to see such a big breaking change. Alternatively, I propose the addition of a new keyword, 
analogous to `async`, for the moment let's use Kotlin's `suspend`. 

Then we could have: 
```rust
suspend fn function() {
    /* 
        implicit await with implicit calls to async destructors
    */
}
async fn function() {
    /* 
        explicit await with explicit calls to async destructors
    */
}

async {
    /* 
        explicit await with explicit calls to async destructors
    */
}

suspend {
    /* 
        implicit await with implicit calls to async destructors
    */
}
```

## related topics
* [io-uring](https://boats.gitlab.io/blog/post/iou/)
* [poll_drop](https://without.boats/blog/poll-drop/)
* [poll_drop_ready](https://internals.rust-lang.org/t/asynchronous-destructors/11127)
* [.await](https://carllerche.com/2021/06/17/six-ways-to-make-async-rust-easier/)
* [async cancellation](https://internals.rust-lang.org/t/blog-post-async-cancellation/15591)
