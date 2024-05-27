use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use hyper::Request;
use tokio::net::TcpStream;
use hyper_util::rt::TokioIo;
use bytes::{Bytes, Buf};
use http_body_util::{Full, BodyExt};

static TOKEN_NAME: &'static str = "TSToken";
static TS_URL: &'static str = "http://api.tushare.pro";

fn get_token()->String {
    std::env::var(TOKEN_NAME).unwrap_or("none".to_string())
}

#[derive(Serialize, Deserialize)]
struct TsRequest {
    api_name: String,
    token: String,
    params: HashMap<String, String>,
    fields: String,
}

impl TsRequest {
    pub fn new(api_name: &'static str)->Self {
        let params = HashMap::new();
        TsRequest {
            api_name: api_name.to_string(),
            token: get_token(),
            params: params,
            fields: String::new(),
        }
    }
    pub fn add_param(&mut self, key: &'static str, value: &'static str) {
        self.params.insert(key.to_string(), value.to_string());
    }
}

async fn test_request()->anyhow::Result<()> {
    // daily data
    let mut daily_data = TsRequest::new("daily");
    daily_data.add_param("ts_code", "000001.SZ");
    daily_data.add_param("start_date", "20240210");
    daily_data.add_param("end_date", "20240410");
    let daily_data_json = serde_json::to_string(&daily_data)?;
    println!("{:?}", daily_data_json);
    let url: hyper::Uri = TS_URL.parse()?;
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr).await?;
    let io: TokioIo<TcpStream> = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let http_req = Request::builder().method("POST").uri(TS_URL)
                                     .header("Content-Type", "application/json")
                                     .body(Full::new(Bytes::from(daily_data_json)))?;
    let res: hyper::Response<hyper::body::Incoming> = sender.send_request(http_req).await?;
    if res.status() == 200 {
        let body = res.collect().await?.aggregate();
        let data = serde_json::from_reader(body.reader())?;
        println!("hello : {:?}", data);
    } else {
        println!("world {:?}", res.status());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::get_token;
    #[test]
    // #[ignore]
    fn token_test() {
        let token = get_token();
        assert_ne!(token, "none".to_string());
    }

    use crate::test_request;
    #[tokio::test]
    async fn request_test()->anyhow::Result<()> {
        test_request().await
    }
}
