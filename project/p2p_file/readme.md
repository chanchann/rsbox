## 传输协议

会使用多种应用层协议，如：HTTP，gRPC及自定义协议等。

## 节点标识

P2P网络中的节点需要一个唯一的标识，以便其他节点能够找到它们。

P2P网络中的节点使用公钥和私钥对(非对称公钥加密)与其他节点建立安全通信

在P2P网络中节点标识被称为PeerId，它是通过对节点公钥进行加密哈希得到的。

## 安全规则

密钥对和节点身份标识

## 节点路由

P2P网络中的一个节点首先需要找到其他节点来进行通信。这是通过维护一个节点路由表来实现的。但是在P2P网络中，有成千上万个节点在动态变化(即节点的加入和离开)，单个节点很难为网络中的所有节点维护一个完整、准确的路由表。所以节点路由表通常会由一系列路由节点维护。

## 消息

P2P网络中的节点可以向特定节点发送消息，也可以广播消息。使用发布/订阅模式，节点订阅感兴趣Topic，所有订阅该Topic的节点都能接收和发送消息。这种技术也通常用于将消息的内容传输到整个网络。

## ping

cargo run --bin ping
