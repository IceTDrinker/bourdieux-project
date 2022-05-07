use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct Agent {
    id: AgentID,
}

#[derive(Debug, Clone, Copy)]
pub struct AgentID(pub usize);

static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn get_agent_id() -> AgentID {
    AgentID(COUNTER.fetch_add(1, Ordering::Relaxed))
}

impl Agent {
    pub fn new() -> Self {
        Self { id: get_agent_id() }
    }

    pub fn id(self: &Self) -> AgentID {
        self.id
    }
}
