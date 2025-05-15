use crate::ip_addr::IpAddr;
use crate::ip_addr_kind::IpAddrKind;

mod ip_addr_kind;
mod ip_addr;

fn main() {
    let local = IpAddr::new(IpAddrKind::V4, String::from("127.0.0.1"));
    let loopback = IpAddr::new(IpAddrKind::V6, String::from("::1"));

    println!("local: {:?}", local);
    println!("loopback: {:?}", loopback);
}
