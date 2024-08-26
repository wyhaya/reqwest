use futures_util::FutureExt;
use reqwest::{ClientBuilder, CustomProxyConnector, CustomProxyStream, Proxy};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let connector = CustomProxyConnector::new(|uri| {
        async move {
            let host = uri.host().unwrap();
            let port = match (uri.scheme_str(), uri.port_u16()) {
                (_, Some(p)) => p,
                (Some("http"), None) => 80,
                (Some("https"), None) => 443,
                _ => unreachable!(),
            };
            let addr = format!("{host}:{port}");
            println!("Connecting to {addr}");
            let stream = TcpStream::connect(addr).await.unwrap();
            stream.set_nodelay(true).unwrap();
            Ok(Box::new(stream) as Box<dyn CustomProxyStream>)
        }
        .boxed()
    });

    let proxy = Proxy::all(connector).unwrap();
    let client = ClientBuilder::new()
        .proxy(proxy)
        .build()
        .unwrap();

    let res = client.get("https://google.com").send().await.unwrap();
    dbg!(res.version());
    dbg!(&res);
    let res = client.get("https://google.com").send().await.unwrap();
    dbg!(res.version());
    dbg!(&res);
}
