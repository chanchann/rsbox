## ref

https://jasonkayzk.github.io/2023/12/27/Rust%E4%B8%AD%E4%BD%BF%E7%94%A8libp2p/

## Libp2p 主要模块

传输（Transport）：负责从一个 peer 到另一个 peer 的数据的实际传输和接收

身份（Identity）：libp2p 使用公钥密钥（PKI）作为 peer 节点身份的基础。使用加密算法为每个节点生成唯一的 peer id。

安全（Security）：节点使用其私钥对消息进行签名。节点之间的传输连接可以升级为安全的加密通道，以便远程 peer 可以相互信任，并且没有第三方可以拦截它们之间的通信。

Peer 发现（Peer Discovery）：允许 peer 在 libp2p 网络中查找并相互通信。

Peer 路由（Peer Routing）：使用其他 peer 的知识信息来实现与 peer 节点的通信。

内容发现（Content Discovery）：在不知道哪个 peer 节点拥有该内容的情况下，允许 peer 节点从其他 peer 节点获取部分内容。

消息（Messaging）：其中发布/订阅：允许向对某个主题感兴趣的一组 peer 发送消息。


## 公钥和私钥



