use crate::ip_addr_kind::IpAddrKind;

#[derive(Debug)]
pub struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddr {
    pub fn new(kind: IpAddrKind, address: String) -> Self {
        IpAddr { kind, address }
    }
}
