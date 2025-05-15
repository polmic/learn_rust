use crate::ip_addr::IpAddr;

mod ip_addr;

fn main() {
    let local = IpAddr::V4(127, 0, 0 ,1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("local: {:?}", local);
    println!("loopback: {:?}", loopback);
}
