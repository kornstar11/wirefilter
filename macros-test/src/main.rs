
use wirefilter_macros::*;
use wirefilter::derive::*;
use wirefilter::*;
use std::net::IpAddr;
use std::time::SystemTime;

#[derive(Debug, Filterable, HasFields)]
#[field(name="empty")]
struct Empty{
    #[field(name="test.a")]
    a: String,
    b: IpAddr,
    c: usize,
    d: Option<String>,
    e: Vec<(String, String)>,
    #[field(ignore="true")]
    f: SystemTime
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
        d: Some("D".to_string()),
        e: vec![("k".to_ascii_lowercase(), "v".to_ascii_lowercase())],
        f: SystemTime::now(),
    };
    e.filter_context(&scheme).unwrap();
    println!("Hello, world!");
}
