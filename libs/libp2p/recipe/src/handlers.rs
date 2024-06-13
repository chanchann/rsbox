use crate::models::{ListMode, ListRequest, ListResponse, Recipe};
use crate::behaviour::{RecipeBehaviour, RecipeBehaviourEvent};
use crate::consts::{PEER_ID, STORAGE_FILE_PATH, TOPIC};

use tokio::sync::mpsc;
use libp2p::Swarm;
use libp2p::swarm::SwarmEvent;
use libp2p::floodsub::FloodsubEvent;
use libp2p::futures::StreamExt;

use log::{debug, error, info};


// pub async fn handle_list_peers(swarm: &mut Swarm<RecipeBehaviour>) {
//     info!("Discovered Peers:");
//     let nodes = swarm.behaviour().mdns.discovered_nodes();

//     let mut unique_peers = HashSet::new();
//     for peer in nodes {
//         unique_peers.insert(peer);
//     }
//     unique_peers.iter().for_each(|p| info!("{}", p));
// }
pub async fn handle_swarm_event(
    response_sender: mpsc::UnboundedSender<ListResponse>,
    swarm: &mut Swarm<RecipeBehaviour>,
) {
    let event = swarm.select_next_some().await;
    info!("Income swarm Event: {:?}", event);

    match event {
        SwarmEvent::Behaviour(recipe_behaviours) => match recipe_behaviours {
            RecipeBehaviourEvent::Floodsub(flood_sub_event) => match flood_sub_event {
                FloodsubEvent::Message(msg) => {
                    if let Ok(resp) = serde_json::from_slice::<ListResponse>(&msg.data) {
                        if resp.receiver == PEER_ID.to_string() {
                            info!("Response from {}:", msg.source);
                            resp.data.iter().for_each(|r| info!("{:?}", r));
                        }
                    } else if let Ok(req) = serde_json::from_slice::<ListRequest>(&msg.data) {
                        match req.mode {
                            ListMode::All => {
                                info!("Received ALL req: {:?} from {:?}", req, msg.source);
                                respond_with_public_recipes(
                                    response_sender.clone(),
                                    msg.source.to_string(),
                                );
                            }
                            ListMode::One(ref peer_id) => {
                                if peer_id == &PEER_ID.to_string() {
                                    info!("Received req: {:?} from {:?}", req, msg.source);
                                    respond_with_public_recipes(
                                        response_sender.clone(),
                                        msg.source.to_string(),
                                    );
                                }
                            }
                        }
                    }
                }
                FloodsubEvent::Subscribed { .. } => {}
                FloodsubEvent::Unsubscribed { .. } => {}
            },
            RecipeBehaviourEvent::Mdns(mdns_event) => match mdns_event {
                Event::Discovered(discovered_list) => {
                    let behavior_mut = swarm.behaviour_mut();
                    for (peer, _addr) in discovered_list {
                        behavior_mut.flood_sub.add_node_to_partial_view(peer);
                    }
                }
                Event::Expired(expired_list) => {
                    let behavior_mut = swarm.behaviour_mut();
                    for (peer, _addr) in expired_list {
                        if !behavior_mut.mdns.has_node(&peer) {
                            behavior_mut.flood_sub.remove_node_from_partial_view(&peer);
                        }
                    }
                }
            },
        },
        SwarmEvent::ConnectionEstablished {
            peer_id,
            connection_id,
            endpoint,
            num_established,
            ..
        } => {
            debug!("[Connection established] peer_id: {}, connection_id: {}, endpoint: {:?}, num_established: {:?}", peer_id, connection_id, endpoint, num_established);
        }
        SwarmEvent::ConnectionClosed {
            peer_id,
            connection_id,
            endpoint,
            num_established,
            ..
        } => {
            debug!("[Connection closed] peer_id: {}, connection_id: {}, endpoint: {:?}, num_established: {:?}", peer_id, connection_id, endpoint, num_established);
        }
        SwarmEvent::IncomingConnection { .. } => {}
        SwarmEvent::IncomingConnectionError { .. } => {}
        SwarmEvent::OutgoingConnectionError { .. } => {}
        SwarmEvent::NewListenAddr { .. } => {}
        SwarmEvent::ExpiredListenAddr { .. } => {}
        SwarmEvent::ListenerClosed { .. } => {}
        SwarmEvent::ListenerError { .. } => {}
        SwarmEvent::Dialing { .. } => {}
    };
}