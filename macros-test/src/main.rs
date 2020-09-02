
use wirefilter_macros::*;
use wirefilter::derive::*;
use wirefilter::*;
use std::net::IpAddr;

#[derive(Debug, Filterable, HasFields)]
struct Empty{
    #[field(name="test.a")]
    a: String,
    b: IpAddr,
    c: usize,
    d: Option<String>,
}

fn main() {
    // let scheme = Scheme!(
    //     a: Bytes,
    //     b: Ip,
    //     c: Int,
    //     d: Bytes
    // );
    // let t = String::ty();
    // let t = Option::<String>::ty();
    let scheme = Empty::fields();
    let scheme = Scheme::try_from_iter(scheme).unwrap();
    let e = Empty{
        a: String::from("A"),
        b: IpAddr::from([1,1,1,1]),
        c: 1234,
        d: Some("D".to_string())
    };
    e.filter_context(&scheme).unwrap();
    println!("Hello, world!");
}
