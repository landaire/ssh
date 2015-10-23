#[test]
fn it_works() {
}

struct TcpAddr<'a> {
    host: &'a str,
    port: u32,
}

impl<'a> TcpAddr<'a> {
    fn new(host: &str, port: u32) -> Result<TcpAddr, &'static str> {
        if port > u16::max_value() as u32 {
            return Err("port out of range")
        }

        Ok(TcpAddr{ host: host, port: port})
    }
}

#[test]
fn test_tcp_adr_parse() {
    match TcpAddr::new("127.0.0.1", u16::max_value() as u32 + 1) {
        Ok(addr)       =>  {
            println!("Works!");
            assert!(addr.host == "127.0.0.1");
        },
        Err(fail)   => println!("Fail: {}", fail),
    }
}


