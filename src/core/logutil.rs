const MOD: u64 = 100000;

fn shard_id(shard_id: u64) -> String {
    format!("c{:05}", shard_id % MOD)
}

fn replica_id(replica_id: u64) -> String {
    format!("n{:05}", replica_id % MOD)
}