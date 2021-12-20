#[macro_use]
extern crate no_await;

#[suspend]
fn func() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.rust-lang.org")?;
    let text = response.text()?;
    println!("{}", text);
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("{:?}", func().await);
}
