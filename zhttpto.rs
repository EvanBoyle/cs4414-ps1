//
// zhttpto.rs
//
// University of Virginia - cs4414 Fall 2013
// Weilin Xu and David Evans
// Version 0.1

extern mod extra;
use std::{os, uint, vec, io, result};

use extra::uv;
use extra::{net_ip, net_tcp};
use std::str;

static BACKLOG: uint = 5;
static PORT:    uint = 4414;
static IPV4_LOOPBACK: &'static str = "127.0.0.1";
static mut visitor_count : int =  0;

unsafe fn new_connection_callback(new_conn :net_tcp::TcpNewConnection, _killch: std::comm::SharedChan<Option<extra::net_tcp::TcpErrData>>)
{
    do spawn {
        let accept_result = extra::net_tcp::accept(new_conn);
        match accept_result {
            Err(err) => {
               println(fmt!("Connection error: %?", err));
            },  
            Ok(sock) => {
                let peer_addr: ~str = net_ip::format_addr(&sock.get_peer_addr());
                println(fmt!("Received connection from: %s", peer_addr));
                
                let read_result = net_tcp::read(&sock, 0u);
                match read_result {
                    Err(err) => {
                        println(fmt!("Receive error: %?", err));
                    },
                     Ok(bytes) => {
                        let request_str = str::from_bytes(bytes.slice(0, bytes.len() - 1));
                        println(fmt!("Request received:\n%s", request_str));
                        let reqtype: ~[&str]= request_str.split_iter(' ').collect();
                        inc_visitors();
                        
                        
                        let response: ~str = match reqtype[1]{
                        	"/"=>
                            fmt!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                             <doctype !html><html><head><title>Hello, Rust!</title>
                             <style>body { background-color: #111; color: #FFEEAA }
                                    h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                             </style></head>
                             <body>
                             <h1>Greetings, Rusty!</h1>
                             <p>Number of Visitors: %d </p>
                             </body></html>\r\n",  get_visitors()),

                        
                        	_=>"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n"+serve_page("." + reqtype[1])
                        
                        
                        
                        };
                        
                        net_tcp::write(&sock, response.as_bytes_with_null_consume());
                        //let path = reqtype.split_iter(' ').advance().next();
                         
                         
                    },
                };
            }
        }
    };
}

fn serve_page(path: ~str)->~str{
	
	let fileReaderTest: Result<@Reader, ~str> = io::file_reader(~PosixPath(path));
	
	if fileReaderTest.is_err(){
		return ~"404 page not found";
	}
	
	let fileReader: @Reader = io::file_reader(~PosixPath(path)).unwrap();
	
	let mut bytes: ~[u8] = ~[];
	loop{
		let byte: int = fileReader.read_byte();
		if fileReader.eof() {break}
		bytes.push(byte as u8);
	}
	
	return str::from_bytes(bytes);
}

unsafe fn inc_visitors(){
	visitor_count+=1;
	
} 
unsafe fn get_visitors()-> int{
	visitor_count
}
fn main() {
    net_tcp::listen(net_ip::v4::parse_addr(IPV4_LOOPBACK), PORT, BACKLOG,
                    &uv::global_loop::get(),
                    |_chan| { println(fmt!("Listening on tcp port %u ...", PORT)); },
                    new_connection_callback);
}
