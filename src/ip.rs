
#[derive(Debug)]
enum IpAddress {
    V4(u8,u8,u8,u8),
    V6(String),
}

pub fn ip_test() {
    let pi = IpAddress::V4(192,168,1,120);
    let loopback = IpAddress::V6(String::from("::1"));
    println!("{:#?}", pi);
    println!("{:#?}", loopback);
}
