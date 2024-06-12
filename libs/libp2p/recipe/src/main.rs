mod consts;

use libp2p::{mdns, noise, tcp, yamux, Swarm};
use libp2p::floodsub::Floodsub;

fn main() {
    /*
        Swarm 是一个管理器，管理所有节点的活动、挂起、连接等等；
        创建一个 Swarm 时大概要配置以下几个内容:
            身份验证（Identity）：身份验证的密钥对（KeyPair）；
            运行时环境（Runtime）：例如 tokio、async_std 等；
            连接类型（Transport）：TCP、UDP，以及连接对应的配置：超时时间、连接验证、多路复用等；
            网络行为（NetworkBehaviour）：网络连接的类型，非常重要的概念，用于定义对等点之间网络行为，例如：FloodSub、mDNS 以及自定义 NetworkBehaviour；
            Swarm 自身配置：Executor 线程池创建、管理等；
    */
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(KEYS.clone())
    .with_tokio()
    .with_tcp(
      tcp::Config::default(),
      noise::Config::new,
      yamux::Config::default,
    )?
    .with_behaviour(|_key| RecipeBehaviour {
      flood_sub: Floodsub::new(*PEER_ID),
      mdns: mdns::tokio::Behaviour::new(mdns::Config::default(), KEYS.public().to_peer_id())
      .expect("can create mdns"),
    })?
    .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(5)))
    .build();
}
