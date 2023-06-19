fn main() {
    enums_value();
    enums_parameter(IpAddrKind::V4);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn enums_value() {
    let four: IpAddrKind = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;
    println!("four: {:?}, six: {:?}", four, six);
}


fn enums_parameter(ip_type: IpAddrKind) {
    println!("ip_type: {:?}", ip_type);
}
