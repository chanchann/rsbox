use std::error::Error;

use libp2p::{
    futures::StreamExt,
    identity,
    ping::{Ping, PingConfig},
    swarm::SwarmEvent,
    Multiaddr,
    PeerId,
    Swarm,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let key_pair = idenity::Keypair::generate_ed25519();

    let peed_id = PeerId::from(key_pair.public());
    println!("Peer ID: {}", peed_id);

    let behavior = Ping::new(PingConfig::new().with_keeo_alive(true));

    let transport = libp2p::development_transport(key_pair).await?;

    let mut swam = Swarm::new(transport, hehavior, peed_id);

    swam.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    if let Some(remote_peer) = std::env::args().nth(1) {
        let remote_peer_multiaddr: Multiaddr = remote_peer.parse()?;
        swam.dial(remote_peer_multiaddr)?;
        println!("Remote node : {}", remote_peer);
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Local listen address: {}", address);
            }
            SwarmEvent::Behavior(event) => println!("{:?}", event),
            _ => {}
        }
    }

}