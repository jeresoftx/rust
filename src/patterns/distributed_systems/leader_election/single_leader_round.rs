#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElectionError {
    LeaderAlreadyElected,
}

#[derive(Debug)]
pub struct RoundElection {
    round: u64,
    leader: Option<String>,
}

impl RoundElection {
    pub fn new(round: u64) -> Self {
        Self {
            round,
            leader: None,
        }
    }

    pub fn claim_leadership(&mut self, node_id: &str) -> Result<String, ElectionError> {
        if self.leader.is_some() {
            return Err(ElectionError::LeaderAlreadyElected);
        }

        let leader = node_id.to_string();
        self.leader = Some(leader.clone());
        Ok(leader)
    }

    pub fn start_round(&mut self, round: u64) {
        if round != self.round {
            self.round = round;
            self.leader = None;
        }
    }

    pub fn current_leader(&self) -> Option<String> {
        self.leader.clone()
    }
}
