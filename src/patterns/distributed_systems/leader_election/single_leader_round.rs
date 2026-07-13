#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Conjunto de estados o errores publicos de `ElectionError` dentro del ejemplo.
pub enum ElectionError {
    /// Variante `LeaderAlreadyElected` del estado o error del ejemplo.
    LeaderAlreadyElected,
}

#[derive(Debug)]
/// Tipo publico `RoundElection` usado por el ejemplo para expresar el dominio del patron.
pub struct RoundElection {
    round: u64,
    leader: Option<String>,
}

impl RoundElection {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(round: u64) -> Self {
        Self {
            round,
            leader: None,
        }
    }

    /// Modela la operacion `claim leadership` dentro del ejemplo del patron.
    pub fn claim_leadership(&mut self, node_id: &str) -> Result<String, ElectionError> {
        if self.leader.is_some() {
            return Err(ElectionError::LeaderAlreadyElected);
        }

        let leader = node_id.to_string();
        self.leader = Some(leader.clone());
        Ok(leader)
    }

    /// Modela la operacion `start round` dentro del ejemplo del patron.
    pub fn start_round(&mut self, round: u64) {
        if round != self.round {
            self.round = round;
            self.leader = None;
        }
    }

    /// Modela la operacion `current leader` dentro del ejemplo del patron.
    pub fn current_leader(&self) -> Option<String> {
        self.leader.clone()
    }
}
