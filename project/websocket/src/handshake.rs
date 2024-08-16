
use std::{
    error::Error,
    io::{self, BufRead, Write},
    collections::BTreeMap,
};
use ring::digest;
use base64::{engine::general_purpose, Engine as _};


/*
`impl BufRead` 作为函数参数类型是Rust中的一种语法，称为"trait对象"或"trait绑定"。这里的具体含义是：

1. 函数接受任何实现了 `BufRead` trait 的类型作为参数。

2. `BufRead` 是一个 trait，定义在标准库中，提供了缓冲读取的方法，如 `read_line()` 和 `read_until()`。

3. 使用 `impl BufRead` 而不是具体类型（如 `BufReader<TcpStream>`）可以让函数更加灵活，能够接受任何实现了 `BufRead` trait 的类型。

4. 这种方式允许我们在不改变函数签名的情况下，传入不同的实现了 `BufRead` 的类型，比如 `BufReader<File>`、`BufReader<Stdin>` 等。

5. 这是Rust的多态性的一种体现，通过trait来实现。

总的来说，`impl BufRead` 表示这个参数可以是任何实现了 `BufRead` trait 的类型，提高了函数的通用性和灵活性。
*/
pub fn handshake(reader: &mut impl BufRead, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let size = reader.read_line(&mut buffer)?;
    // 读取 http 请求行
    let request_line: &str = &buffer[0..size];
    println!("received: {}", request_line);
    let _ = request_line;
    buffer.truncate(0);

    let mut headers = BTreeMap::<String, String>::new();

    loop {
        let size = reader.read_line(&mut buffer)?;
        // 读取每一个头信息
        let header_line: &str = &buffer[0..size];
        // 头信息完结
        if header_line == "\r\n" {
            break;
        }

        let header_line = &header_line[0..(size - 2)];
        /*
        `v.trim_start().into()` 使用 `into()` 是为了将 `&str` 转换为 `String`。
        `trim_start()` 返回一个 `&str` 切片，而 `headers` 的值类型是 `String`。
        `into()` 方法在这里用于执行这个类型转换，将 `&str` 转换为 `String`，以匹配 `BTreeMap` 中值的类型。
        这种写法简洁地完成了去除前导空白并转换为所需类型的操作。
         */
        if let Some((k, v)) = header_line.split_once(':') {
            headers.insert(k.to_lowercase(), v.trim_start().into());
        };

        buffer.truncate(0);
    }

    let sec_websocket_key = headers.get("sec-websocket-key").ok_or(io::Error::new(
        io::ErrorKind::ConnectionRefused,
        "no header Sec-Websocket-Key",
    ))?;

    const UUID: &[u8] = b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

    // sha1 加 base64
    let concat_str = [sec_websocket_key.as_bytes(), UUID].concat();
    let hash_result = digest::digest(&digest::SHA1_FOR_LEGACY_USE_ONLY, &concat_str);
    let sec_websocket_accept = general_purpose::STANDARD.encode(hash_result.as_ref());

    let response = format!(
        "HTTP/1.1 101 Switching Protocols\r\n\
        Upgrade: websocket\r\n\
        Connection: Upgrade\r\n\
        Sec-WebSocket-Accept: {}\r\n\r\n",
        sec_websocket_accept
    );

    writer.write_all(response.as_bytes())?;

    writer.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_handshake() {
        let request = "GET / HTTP/1.1\r\n\
                       Host: server.example.com\r\n\
                       Upgrade: websocket\r\n\
                       Connection: Upgrade\r\n\
                       Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
                       Origin: http://example.com\r\n\
                       Sec-WebSocket-Protocol: chat, superchat\r\n\
                       Sec-WebSocket-Version: 13\r\n\r\n";

        let mut reader = Cursor::new(request);
        let mut writer = Vec::new();

        handshake(&mut reader, &mut writer).unwrap();

        let response = String::from_utf8(writer).unwrap();
        assert!(response.contains("HTTP/1.1 101 Switching Protocols"));
        assert!(response.contains("Upgrade: websocket"));
        assert!(response.contains("Connection: Upgrade"));
        assert!(response.contains("Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo="));
    }
}