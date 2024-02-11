use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
};

use local_ip_address::local_ip;

pub struct HttpServer {
    pub listener: TcpListener,
    web_root: PathBuf,
}

impl HttpServer {
    pub fn new() -> HttpServer {
        let listener = TcpListener::bind(format!("{}:80", local_ip().unwrap()))
            .expect("Couldn't open TCP port");
        println!("Listening on {}", listener.local_addr().unwrap());
        let web_root: PathBuf = Path::new(".")
            .join("web")
            .canonicalize()
            .expect("Couldn't find web directory");
        HttpServer {
            listener: listener,
            web_root: web_root,
        }
    }

    pub fn handle(&self) {
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let buf_reader = BufReader::new(&mut stream);
            let http_request: Vec<_> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();

            if http_request.is_empty() {
                return;
            }
            
            if !http_request[0].starts_with("GET") {
                return;
            }
            let http_find = match http_request[0].find("HTTP") {
                Some(n) => n - 1,
                None => return,
            };

            let http_path = &http_request[0][5..http_find];

            if http_request[0].starts_with("GET") {
                Self::handle_get(&self, &mut stream, http_path);
            } else if http_request[0].starts_with("POST") {
                Self::handle_post();
            } else if http_request[0].starts_with("PUT") {
                Self::handle_put();
            } else if http_request[0].starts_with("DELETE") {
                Self::handle_delete();
            } else if http_request[0].starts_with("HEAD") {
                Self::handle_head();
            } else if http_request[0].starts_with("OPTIONS") {
                Self::handle_options();
            } else if http_request[0].starts_with("TRACE") {
                Self::handle_trace();
            } else if http_request[0].starts_with("CONNECT") {
                Self::handle_connect();
            } else if http_request[0].starts_with("PATCH") {
                Self::handle_patch();
            }
        }
    }

    fn handle_get(&self, stream: &mut TcpStream, path: &str) {
        if let Ok(path) = fs::canonicalize(self.web_root.join(path)) {
            let path = &path;
            let mut response = String::from("HTTP/1.1 200 OK\r\n\r\n");

            if !path.starts_with(&self.web_root) {
                // Path traversal
                response = String::from("HTTP/1.1 403 OK\r\n");
                stream.write_all(response.as_bytes()).unwrap();
                return;
            }

            if path.is_dir() {
                // If dir, read index.html

                if let Ok(file_str) =
                    fs::read_to_string(self.web_root.join(path).join("index.html"))
                {
                    // If there is a index.html file, send it
                    response.push_str(&file_str);
                    let _ = stream.write_all(response.as_bytes());
                    return;
                }

                if let Ok(notfound_str) = fs::read_to_string(self.web_root.join("404.html")) {
                    // If there is no index.html but a 404.html at root, try sending it.
                    response.push_str(&notfound_str);
                    let _ = stream.write_all(response.as_bytes());
                    return;
                } else {
                    // If there is no 404 supplied, print message and send 404 as response
                    response = String::from("HTTP/1.1 404 OK\r\n");
                    stream.write_all(response.as_bytes()).unwrap();
                    return;
                }
            }

            if path.is_file() {
                if let Ok(file_str) = fs::read_to_string(self.web_root.join(path)) {
                    response.push_str(&file_str);
                    let _ = stream.write_all(response.as_bytes());
                    return;
                } else {
                    // If file is binary, send it as attachment
                    let mut data = fs::File::open(path).unwrap();
                    let metadata = fs::metadata(path).unwrap();

                    /* FIXME response format! */
                    let response = format!("HTTP/1.1 200 OK\r\nAccept-Ranges: bytes\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\nContent-Disposition: attachment;\r\n\r\n", metadata.len()).to_owned().as_bytes().to_vec();
                    stream.write_all(&response).unwrap();
                    let mut buffer = [0; 4096];
                    while let Ok(n) = data.read(&mut buffer) {
                        if n == 0 {
                            break;
                        }
                        match stream.write_all(&buffer[..n]) {
                            Ok(_) => (),
                            Err(_) => return,
                        };
                    }
                }
            }
        } else {
            // If path is not found, send 404.html
            let mut response = String::from("HTTP/1.1 200 OK\r\n\r\n");
            if let Ok(notfound_str) = fs::read_to_string(self.web_root.join("404.html")) {
                // If there is no index.html but a 404.html at root, try sending it.
                response.push_str(&notfound_str);
                let _ = stream.write_all(response.as_bytes());
                return;
            } else {
                // If there is no 404 supplied, print message and send 404 as response
                response = String::from("HTTP/1.1 404 OK\r\n");
                stream.write_all(response.as_bytes()).unwrap();
                return;
            }
        }
    }

    fn handle_post() {
        unimplemented!();
    }

    fn handle_put() {
        unimplemented!();
    }

    fn handle_delete() {
        unimplemented!();
    }

    fn handle_head() {
        unimplemented!();
    }

    fn handle_options() {
        unimplemented!();
    }

    fn handle_trace() {
        unimplemented!();
    }

    fn handle_connect() {
        unimplemented!();
    }

    fn handle_patch() {
        unimplemented!();
    }
}
