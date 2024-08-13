// 压缩类型
pub enum CompressionType{
    NoCompression, // 不压缩
    Snappy,//精简压缩
}

// 消息类型
enum MessageType {
    LocalTick,          //表示本地的时钟信号或时间戳消息。
    Election,           //选举相关的消息，用于发起或响应领导者选举。
    LeaderHeartbeat,    //领导者发送的心跳消息，用于向其他节点表明自己的活跃状态。
    ConfigChangeEvent,  //配置变更事件消息，用于通知节点关于集群配置的变化。
    NoOP,               //无操作消息，用于占位或表示没有实际的操作要执行。
    Ping,               //类似于网络中的 Ping 消息，用于检测节点的可达性。
    Pong,               //对 Ping 消息的响应。
    Propose,            //提议消息，用于向集群提交新的事务或操作。
    SnapshotStatus,     //关于快照状态的消息，用于通知其他节点快照的生成或应用情况。
    Unreachable,        //表示某个节点不可达的消息。
    CheckQuorum,        //检查法定人数的消息，用于确认集群中具有足够的活跃节点以继续运行。
    BatchedReadIndex,   //批量读取索引的消息，用于优化读取操作。
    Replicate,          //复制消息，用于将数据从一个节点复制到另一个节点。
    ReplicateResp,      //对复制消息的响应。
    RequestVote,        //请求投票的消息，在选举过程中使用。
    RequestVoteResp,    //对请求投票消息的响应。
    InstallSnapshot,    //安装快照的消息，用于将快照数据应用到节点上。
    Heartbeat,          //心跳消息，与 LeaderHeartbeat 类似。
    ReadIndex,          //读取索引的消息，用于获取特定数据的索引位置。
    ReadIndexResp,      //对读取索引消息的响应。
    Quiesce,            //用于使节点进入安静状态的消息。
    SnapshotReceived,   //表示接收到快照的消息。
    LeaderTransfer,     //领导者转移的消息，用于将领导权从一个节点转移到另一个节点。
    TimeoutNow,         //立即超时的消息，用于触发特定的超时操作。
    RateLimit,          //速率限制消息，用于控制消息的发送速率。
    RequestPreVote,     //请求预投票的消息，在选举过程中的特定阶段使用。
    RequestPreVoteResp, //对请求预投票消息的响应。
    LogQuery,           //日志查询消息，用于查询节点的日志记录。
}
