use libp2p::floodsub::{Floodsub, FloodsubEvent};
use libp2p::mdns;
use libp2p::swarm::NetworkBehaviour;

// 此宏会自动实现部分逻辑，同时要求被标注的结构体只能含有实现了 NetworkBehaviour 的属性！
#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "RecipeBehaviourEvent")]  // BehaviourEvent 的定义被抽取到了枚举中，通过这个宏来声明；

pub struct RecipeBehaviour {
    pub(crate) flood_sub: Floodsub,  // 处理消息广播
    pub(crate) mdns: mdns::tokio::Behaviour,  // 节点自动发现
}

#[derive(Debug)]
pub enum RecipeBehaviourEvent {
    Floodsub(FloodsubEvent),
    Mdns(mdns::Event),
}

// 我们通过将 RecipeBehaviour 中定义的行为产生的事件统一使用 From Trait 转换为RecipeBehaviourEvent 来统一处理
impl From<FloodsubEvent> for RecipeBehaviourEvent {
    fn from(event: FloodsubEvent) -> RecipeBehaviourEvent {
        RecipeBehaviourEvent::Floodsub(event)
    }
}

impl From<mdns::Event> for RecipeBehaviourEvent {
    fn from(event: mdns::Event) -> RecipeBehaviourEvent {
        RecipeBehaviourEvent::Mdns(event)
    }
}