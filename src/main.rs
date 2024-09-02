use core::raft::Raft;

use config::Config;

mod config;
mod core;
mod raftpb;
mod error;
fn main() {
    let config = Config{
        replica_id: todo!(),
        shard_id: todo!(),
        check_quorum: todo!(),
        pre_vote: todo!(),
        election_rtt: todo!(),
        heartbeat_rtt: todo!(),
        snapshot_entries: todo!(),
        compaction_overhead: todo!(),
        ordered_config_change: todo!(),
        max_in_mem_log_size: todo!(),
        snapshot_compression_type: todo!(),
        entry_compression_type: todo!(),
        disable_auto_compactions: todo!(),
        is_non_voting: todo!(),
        is_witness: todo!(),
        quiesce: todo!(),
        wait_ready: todo!(),
    };
    let _ = Raft::new(config);
}
