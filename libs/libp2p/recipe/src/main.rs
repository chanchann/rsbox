mod consts;
mod behaviour;
mod models;
mod handlers;

use crate::behaviour::RecipeBehaviour;
use crate::consts::{KEYS, PEER_ID, TOPIC};
use crate::models::EventType;
use crate::handlers::handle_swarm_event;

use std::time::Duration;
use std::env;

use libp2p::{mdns, noise, tcp, yamux, Swarm};
use libp2p::floodsub::Floodsub;

use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc;

use log::{error, info};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    info!("Peer Id: {}", PEER_ID.clone());
    let (response_sender, mut response_rcv) = mpsc::unbounded_channel();
    /*
        Swarm 是一个管理器，管理所有节点的活动、挂起、连接等等；
        创建一个 Swarm 时大概要配置以下几个内容:
            身份验证（Identity）：身份验证的密钥对（KeyPair）；
            运行时环境（Runtime）：例如 tokio、async_std 等；
            连接类型（Transport）：TCP、UDP，以及连接对应的配置：超时时间、连接验证、多路复用等；
            网络行为（NetworkBehaviour）：网络连接的类型，非常重要的概念，用于定义对等点之间网络行为，例如：FloodSub、mDNS 以及自定义 NetworkBehaviour；
            Swarm 自身配置：Executor 线程池创建、管理等；
    */
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(KEYS.clone())  // 先指定密钥
    .with_tokio()  // 指定异步运行时为tokio
    .with_tcp(  // 使用 TCP 为 Transport
      tcp::Config::default(),  // TCP 配置，例如连接超时时间等；
      noise::Config::new,  // TCP 安全连接配置，这里使用 noise 握手，即使用密钥对的方式；
      yamux::Config::default,  // 连接多路复用配置，这里使用 yamux；
    )?
    .with_behaviour(|_key| RecipeBehaviour {  // 该 Swarm 网络行为定义，这里为自定义的 RecipeBehaviour
      flood_sub: Floodsub::new(*PEER_ID),
      mdns: mdns::tokio::Behaviour::new(mdns::Config::default(), KEYS.public().to_peer_id())
      .expect("can create mdns"),
    })?
    .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(5)))  //  自身配置，这里作为实验，配置了闲置超时时间为 5s
    .build();

    Swarm::listen_on(
        &mut swarm,
        "/ip4/0.0.0.0/tcp/0"  // 监听 IPV4 下任意 IP 来源（0.0.0.0）的连接，并且监听端口由系统分配（端口号为0）；
            .parse()
            .expect("can get a local socket"),
    )
    .expect("swarm can be started");

    swarm.behaviour_mut().flood_sub.subscribe(TOPIC.clone());  // 启动了 Swarm 中定义的 FloodSub 对我们 Topic 的监听：

    // 处理输入请求

    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();
    loop {
        let evt = {
            tokio::select! {
                line = stdin.next_line() => Some(EventType::Input(line.expect("can get line").expect("can read line from stdin"))),  // 用户完成了一行数据
                response = response_rcv.recv() => Some(EventType::Response(response.expect("response exists"))),  // 列表数据请求的响应
                _ = handle_swarm_event(response_sender.clone(), &mut swarm) => None,  // 处理到来的 Swarm 事件
            }
        };

        // if let Some(event) = evt {
        //     match event {
        //         EventType::Response(resp) => {
        //             let json = serde_json::to_string(&resp).expect("can jsonify response");
        //             swarm
        //                 .behaviour_mut()
        //                 .flood_sub
        //                 .publish(TOPIC.clone(), json.as_bytes());
        //         }
        //         EventType::Input(line) => match line.as_str() {
        //             "ls p" => handle_list_peers(&mut swarm).await,
        //             cmd if cmd.starts_with("create r") => handle_create_recipe(cmd).await,
        //             cmd if cmd.starts_with("publish r") => handle_publish_recipe(cmd).await,
        //             cmd if cmd.starts_with("ls r") => handle_list_recipes(cmd, &mut swarm).await,
        //             _ => error!("unknown command: {:?}", line),
        //         },
        //     }
        // }
    }

    Ok(())
}
