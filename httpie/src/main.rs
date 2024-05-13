use clap::Parser;
use reqwest::Url;
use anyhow::{anyhow, Ok, Result};

/// A naive httpie implementation wite Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// 子命令分别对应不同的 HTTP 方法，暂时只支持 GET / POST 方法
#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
    // TODO: other http methods
}

/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的 URL
    #[arg(value_parser=parse_url)]
    url: String,
}

#[derive(Parser, Debug)]
struct Post{
    /// HTTP 请求的 URL
    url: String,
    /// HTTP 请求的 body
    body: Vec<String>,
}

fn parse_url(s: &str) -> Result<String> {
    // check url
    let _url: Url = s.parse()?;

    Ok(s.into())
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
