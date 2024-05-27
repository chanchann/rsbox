use std::collections::HashMap;

async fn test_basic() -> anyhow::Result<()> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

async fn test_get() -> anyhow::Result<()> {
    let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;

    println!("body_size = {}", body.len());
    Ok(())
}

async fn test_post() -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let res = client.post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await?;
    println!("{}", res.text().await?);
    Ok(())
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    test_basic().await?;
    test_get().await?;
    test_post().await?;
    Ok(())

}
