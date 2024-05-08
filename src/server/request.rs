use std::collections::HashMap;
use std::{
    io::{prelude::*, BufReader}, 
    net::TcpStream,
};

use anyhow::Error;

use super::http::HttpMethod;

#[derive(Debug)]
pub struct Path {
    pub path: String,
    pub query: String,
}

impl Path {
    pub fn build (path: String) -> Path {
        let mut split = path.split('?');
        let path = split.next().unwrap().to_owned();
        let query = split.next().unwrap_or_default().to_owned();
        
        Path{ path, query }
    }
}


#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub path: Path,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn parse(stream: &TcpStream) -> Result<Request, Error> {
        let mut buf_reader = BufReader::new(stream);

        let (method, path) = Request::parse_status_line(&mut buf_reader)?;

        let headers = Request::parse_headers(&mut buf_reader)?;

        let body = Request::parse_body(
            &mut buf_reader, 
            headers.get("Content-Length").unwrap_or(&String::from("0")),
        );

        let request = Request { method, path, headers, body };

        Ok(request)
    }

    fn parse_status_line(buf_reader: &mut BufReader<&TcpStream>) -> Result<(HttpMethod, Path), Error> {

        let mut status_line = String::new();
        buf_reader.read_line(&mut status_line)?;

        let mut parts = status_line.split(' ');

        let method = HttpMethod::parse(parts.next().unwrap());
        let path = Path::build(parts.next().unwrap().to_owned());

        Ok((method, path))
    }

    fn parse_headers(buf_reader: &mut BufReader<&TcpStream>) -> Result<HashMap<String, String>, Error> {
        let mut headers = HashMap::new();

        let mut header_line = String::new();
        loop {
            buf_reader.read_line(&mut header_line)?;

            match header_line == "\r\n" {
                true => break,
                false => {
                    let (key, val) = header_line.split_once(':').unwrap();         
                    headers.insert(
                        clean_header_value(key), 
                        clean_header_value(val),
                    );
                }
            }
            header_line.clear();
        }

        Ok(headers)
    }

    fn parse_body(buf_reader: &mut BufReader<&TcpStream>, content_len: &String) -> String {
        let len = content_len.parse::<u64>().unwrap();
        let default = String::from("0");

        if len > 0 {
            let mut buf = buf_reader.take(len);
            let mut body = vec![];
            match buf.read_to_end(&mut body) {
                Ok(_) => String::from_utf8_lossy(&body).to_string(),
                Err(_) => default,
            }
        } else {
            default
        }
    }
}


fn clean_header_value(val: &str) -> String {
    val.trim().to_owned()
}
