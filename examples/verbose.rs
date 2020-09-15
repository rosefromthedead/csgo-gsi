use csgo_gsi::{GSIConfigBuilder, GSIServer, Subscription};

#[tokio::main]
async fn main() {
    let config = GSIConfigBuilder::new("csgo-gsi Example")
        .subscribe_multiple(Subscription::UNRESTRICTED)
        .build();

    let mut server = GSIServer::new(config, 31337);
    server.add_listener(|update| println!("Got an update {:#?}", update));

    server
        .run()
        .await
        .expect("server didn't start");
}
