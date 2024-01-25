use url::Url;
use tungstenite::{connect, Message};
pub fn connect_to_socket() {
    // TODO need to sub to an event or this will auto close
    let (mut socket, response) = connect(
        Url::parse("wss://eventsub.wss.twitch.tv/ws").unwrap()
    ).expect("Can't connect");

    let mut i = 0;
    while i < 10 {
        let x = socket.read();
        println!("{:?}", x);
        i+=1;
    }
}