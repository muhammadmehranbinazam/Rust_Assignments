#[derive(Debug)]
enum IpAddress{
    ipV2,
    ipV6,
}
#[derive(Debug)]
struct IPAddress{
    kind:IpAddress,
    address:String,
}
fn main() {
    let ip=IPAddress{
        kind:IpAddress::ipV6,
        address:String::from("19.fd.er.er.er")
    };
    println!("{:#?}",ip);
}
