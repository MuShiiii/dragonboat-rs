use crate::raftpb::types::CompressionType;
struct Config {
    // replica_id 是一个非零值，用于在 Raft 分片内标识一个节点。
    replica_id: u64,

    // shard_id 是用于标识包含多个副本的 Raft 组的唯一值。
    shard_id: u64,
    // check_quorum 指定领导者节点是否应定期检查非领导者节点状态，并在不再拥有法定人数时降级为跟随者节点。

    check_quorum: bool,
    // 是否为这个节点使用预投票（PreVote）。预投票在 Raft 论文的 9.7 节中有描述。
    pre_vote: bool,

    // ElectionRTT 是选举之间的最小消息往返时间（RTT）数量。消息 RTT 由 NodeHostConfig.RTTMillisecond 定义。Raft 论文建议它比心跳 RTT 大一个数量级，心跳 RTT 是两次心跳之间的时间间隔。在 Raft 中，实际选举间隔是在 ElectionRTT 和 2 * ElectionRTT 之间随机化的。
    //
    // 例如，假设 NodeHostConfig.RTTMillisecond 是 100 毫秒，要将选举间隔设置为 1 秒，那么 ElectionRTT 应设置为 10。
    //
    // 当 CheckQuorum 启用时，ElectionRTT 也定义了检查领导者法定人数的时间间隔。
    election_rtt: u64,

    // HeartbeatRTT 是心跳之间的消息往返时间数量。消息 RTT 由 NodeHostConfig.RTTMillisecond 定义。Raft 论文建议心跳间隔接近节点之间的平均 RTT。
    //
    // 例如，假设 NodeHostConfig.RTTMillisecond 是 100 毫秒，要将心跳间隔设置为每 200 毫秒一次，那么 HeartbeatRTT 应设置为 2。
    heartbeat_rtt: u64,

    // SnapshotEntries 定义了状态机应多久自动进行一次快照。它以应用的 Raft 日志条目数量来定义。将 SnapshotEntries 设置为 0 可禁用这种自动快照功能。
    //
    // 当 SnapshotEntries 设置为 N 时，这意味着大约每 N 个应用的 Raft 日志条目（提案）创建一个快照。这也意味着向跟随者发送 N 个日志条目比发送一个快照更昂贵。
    //
    // 一旦生成快照，新快照覆盖的 Raft 日志条目可以被压缩。这涉及两个步骤，首先将冗余的日志条目标记为已删除，然后在稍后阶段发出 LogDB 压缩时从底层存储中物理删除它们。有关生成快照后实际删除和压缩哪些日志条目的详细信息，请参阅 CompactionOverhead 的 godoc。
    //
    // 一旦通过将 SnapshotEntries 字段设置为 0 禁用自动快照，用户仍然可以使用 NodeHost 的 RequestSnapshot 或 SyncRequestSnapshot 方法手动请求快照。
    snapshot_entries: u64,

    // CompactionOverhead 定义了每次 Raft 日志压缩后要保留的最新条目的数量。每次创建快照时都会自动执行 Raft 日志压缩。
    //
    // 例如，当在索引 10000 处创建快照时，那么该节点上所有索引 <= 10000 的 Raft 日志条目都可以被删除，因为它们已经被创建的快照图像覆盖。这释放了最大的存储空间，但代价是如果跟随者需要任何索引 <= 10000 的 Raft 日志条目，则必须发送完整的快照。当 CompactionOverhead 设置为比如说 500 时，Dragonboat 会将 Raft 日志压缩到索引 9500，并保留索引在 (9500, 10000] 之间的 Raft 日志条目。因此，节点仍然可以将索引在 (9500, 10000] 之间的 Raft 日志条目复制到其他对等节点，并且只有在需要复制索引 <= 9500 的任何 Raft 日志条目时才回退到流式传输完整快照。
    compaction_overhead: u64,

    // OrderedConfigChange 确定是否使用有序的配置更改 ID 强制执行 Raft 成员变更。
    //
    // 当设置为 true 时，成员变更请求需要 ConfigChangeIndex。这表现得像一个乐观写锁，强制客户端明确地线性化成员变更请求。（推荐）
    //
    // 当设置为 false（默认值）时，成员变更请求忽略 ConfigChangeIndex。这可能导致客户端基于过时的成员数据请求成员变更。

    ordered_config_change: bool,
    // MaxInMemLogSize 是每个 Raft 节点上允许用于存储内存中 Raft 日志的目标大小（以字节为单位）。内存中的 Raft 日志是尚未应用的日志。
    // MaxInMemLogSize 是一个目标值，用于防止内存无限制增长，它不是用于精确限制确切的内存使用量。
    // 当 MaxInMemLogSize 为 0 时，目标设置为 math.MaxUint64。当设置了 MaxInMemLogSize 并且达到目标时，当客户端尝试进行新提案时将返回错误。
    // MaxInMemLogSize 建议设置为比你将要使用的最大提案大得多的值。

    max_in_mem_log_size: u64,
    // SnapshotCompressionType 是用于压缩生成的快照数据的压缩类型。默认情况下不使用压缩。

    snapshot_compression_type: CompressionType,
    // EntryCompressionType 是用于压缩用户提案的有效负载的压缩类型。当使用 Snappy 时，允许的最大提案有效负载大约限制为 3.42GB。默认情况下不使用压缩。

    entry_compression_type: CompressionType,
    
    // DisableAutoCompactions 禁用用于回收 Raft 日志条目存储空间的自动压缩。默认情况下，每次创建快照时都会发出压缩请求，这有助于尽快回收磁盘空间，但代价是立即增加 IO 开销。用户可以禁用这种自动压缩，并在必要时使用 NodeHost.RequestCompaction 手动请求这种压缩。
    disable_auto_compactions: bool,
    // IsNonVoting 指示这是否是一个无投票权的 Raft 节点。在 Diego Ongaro 的论文的 4.2.1 节中描述为非投票成员，它们用于允许新节点加入分片并赶上其他现有节点，而不会影响可用性。还可以引入额外的非投票节点来服务只读请求。
    
    is_non_voting: bool,
   
    // IsWitness 指示这是否是一个见证 Raft 节点，没有实际的日志复制并且没有状态机。在 Diego Ongaro 的论文的 11.7.2 节中提到。
    //
    // 见证支持目前处于实验阶段。
    is_witness: bool,
    
    // Quiesce 指定当没有分片活动时是否让 Raft 分片进入静止模式。处于静止模式的分片不交换心跳消息以最小化带宽消耗。
    //
    // 静止支持目前处于实验阶段。
    quiesce: bool,
    
    // WaitReady 指定是否在从 StartReplica 返回之前等待节点从恢复状态转换为就绪状态。
    wait_ready: bool,
}
