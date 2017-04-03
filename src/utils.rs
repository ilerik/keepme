use hyper::client::{Client};
use tokio_core::reactor::{Core};
use hyper::net::HttpsConnector;
use futures::future::Future;
use hyper_native_tls::NativeTlsClient;
use std::io::{Read};

// Error type
pub use std::error::Error as Error;

pub fn s(buf: &[u8]) -> &str {
    ::std::str::from_utf8(buf).unwrap()
}

pub fn splitter(s: &str) -> Vec<&str> {
    s.split('/').filter(|x| !x.is_empty()).collect()
}

pub fn get_content(url: &str) -> impl Future<Item = String, Error = ()>{
    let url = hyper::Url::parse(url).unwrap();
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    let mut response = client.get(url).send().unwrap();
    println!("{}", response.headers);

    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    //let v : Value = serde_json::from_slice(&.body()[..]).unwrap();
    futures::future::ok(body)
}

#[cfg(test)]
mod tests {
    use std::str;
    use tokio_core::reactor::{Core};
    use futures::future::Future;

    #[test]
    fn test_get_content() {
        pub use utils::get_content;
        let mut core = Core::new().unwrap();
        let body = core.run(get_content("https://google.com")).unwrap();
        println!("{}", body);
        assert_eq!( body, "" );
    }
}
