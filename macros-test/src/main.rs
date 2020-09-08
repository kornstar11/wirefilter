use wirefilter_macros::*;
use wirefilter::derive::*;
use wirefilter::*;
use std::net::IpAddr;
use std::time::SystemTime;

fn main() {
    println!("use cargo test");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Filterable, HasFields)]
    #[field(name="empty")]
    struct Test1 {
        #[field(name="test.a")]
        a: String,
        b: IpAddr,
        c: usize,
        d: Option<String>,
        e: Vec<(String, String)>,
        #[field(ignore="true")]
        f: SystemTime
    }

    #[test]
    fn handle_renamed_fields() {
        let scheme = Test1::fields();
        let scheme = Scheme::try_from_iter(scheme).unwrap();
        let e = Test1 {
            a: String::from("A"),
            b: IpAddr::from([1,1,1,1]),
            c: 1234,
            d: Some("D".to_string()),
            e: vec![("k".to_ascii_lowercase(), "v".to_ascii_lowercase())],
            f: SystemTime::now(),
        };
        e.filter_context(&scheme).unwrap();
    }
}
