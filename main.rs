use std::io::{Write,Read,BufRead};
fn main() {
   let listener = std::net::TcpListener::bind("0.0.0.0:3454").unwrap();
   loop {
    let stream = listener.accept().unwrap().0;
    handle_func(stream);
   }
}

fn handle_func(stream: std::net::TcpStream) {
    let mut r = std::io::BufReader::new(stream);
        {
        for ln in r.by_ref().lines() {
       
            {
                    match ln {
                            Ok(val)=> {
                                if val=="" {break;}
                                else{println!("{}",val)}
                            },
                            Err(e)=>println!("{:?}",e)
                    };
            }
        }
    send_response(r.into_inner());
    }
}

fn send_response(mut stream: std::net::TcpStream) {
    let response = "HTTP/1.1 200 OK\n\n<html><body>Hello Rust!</body></html>";
    stream.write_all(response.as_bytes()).unwrap();
}