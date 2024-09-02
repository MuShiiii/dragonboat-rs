use super::logentry;
use crate::error::DBRError;
use crate::{config::Config, raftpb};
use std::{borrow::Borrow, rc::Rc};

// raft 节点类型
enum State {
    Leader,
    Follower,
    Candidate,
    PreVoteCandidate,
    NonVoting,
    Witness,
}

// raft 节点状态
struct Status {
    replica_id: u64,
    shard_id: u64,
    applied: u64,
    leader_id: u64,
    node_state: Rc<State>,
    pb_status: raftpb::state::State,
}

fn is_leader(status: &Status) -> bool {
    matches!(status.node_state.borrow(), State::Leader)
}

fn is_follower(status: &Status) -> bool {
    matches!(status.node_state.borrow(), State::Follower)
}

fn is_candidate(status: &Status) -> bool {
    matches!(status.node_state.borrow(), State::Candidate)
}

fn is_pre_vote_candidate(status: &Status) -> bool {
    matches!(status.node_state.borrow(), State::PreVoteCandidate)
}

fn is_non_voting(status: &Status) -> bool {
    matches!(status.node_state.borrow(), State::NonVoting)
}

fn is_witness(status: &Status) -> bool {
    matches!(status.node_state.borrow(), State::Witness)
}

// raft 结构体
pub struct Raft {
    // handlers: [[HandlerFunc; num_message_types]; num_states],
    // events: Option<IRaftEventListener>,
    // has_not_applied_config_change: Option<fn() -> bool>,
    // votes: HashMap<u64, bool>,
    // handle: Option<StepFunc>,
    log: Option<logentry::EntryLog>,
    // rl: Option<InMemRateLimiter>,
    // remotes: HashMap<u64, Remote>,
    // non_votings: HashMap<u64, Remote>,
    // witnesses: HashMap<u64, Remote>,
    // log_query_result: Option<LogQueryResult>,
    // leader_update: Option<LeaderUpdate>,
    // read_index: Option<ReadIndex>,
    // matched: Vec<u64>,
    // msgs: Vec<Message>,
    // dropped_read_indexes: Vec<SystemCtx>,
    // dropped_entries: Vec<Entry>,
    // ready_to_read: Vec<ReadyToRead>,
    // prev_leader: LeaderInfo,
    state: Rc<State>, //节点类型
    leader_transfer_target: u64,
    leader_id: u64,
    shard_id: u64,
    replica_id: u64,
    term: u64,
    applied: u64,
    vote: u64,
    tick_count: u64,
    election_tick: u64,
    heartbeat_tick: u64,
    heartbeat_timeout: u64,
    election_timeout: u64,
    randomized_election_timeout: u64,
    snapshotting: bool,
    check_quorum: bool,
    quiesce: bool,
    is_leader_transfer_target: bool,
    pending_config_change: bool,
    pre_vote: bool,
}

fn getLocalStatus(raft: &Raft) -> Status {
    Status {
        replica_id: raft.replica_id,
        shard_id: raft.shard_id,
        applied: raft.applied,
        leader_id: raft.leader_id,
        node_state: raft.state.clone(),
        pb_status: raftStatus(raft),
    }
}

fn raftStatus(raft: &Raft) -> raftpb::state::State {
    raftpb::state::State {
        term: raft.term,
        vote: raft.vote,
        commit: raft.log.as_ref().unwrap().commited,
    }
}

impl Raft {

    pub fn new(config: Config) -> Result<Raft, DBRError> {

        config.validate().unwrap();
        todo!()
    
    }
}

