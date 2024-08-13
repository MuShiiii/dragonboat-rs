// raft protobuf state
pub struct State{
    pub  term : u64,
    pub vote : u64,
    pub commit : u64
}
