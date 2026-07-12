#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JitteredBackoffPolicy {
    initial_delay_ms: u64,
    max_jitter_ms: u64,
}

impl JitteredBackoffPolicy {
    pub fn new(initial_delay_ms: u64, max_jitter_ms: u64) -> Self {
        Self {
            initial_delay_ms,
            max_jitter_ms,
        }
    }

    fn delay_for(&self, retry_index: usize, node_id: &str) -> u64 {
        let base_delay = self.initial_delay_ms * 2_u64.pow(retry_index as u32);
        base_delay + deterministic_jitter(node_id, self.max_jitter_ms)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrySchedule {
    node_id: String,
    policy: JitteredBackoffPolicy,
}

impl RetrySchedule {
    pub fn new(node_id: impl Into<String>, policy: JitteredBackoffPolicy) -> Self {
        Self {
            node_id: node_id.into(),
            policy,
        }
    }

    pub fn delays_for_retries(&self, retries: usize) -> Vec<u64> {
        (0..retries)
            .map(|retry_index| self.policy.delay_for(retry_index, &self.node_id))
            .collect()
    }
}

fn deterministic_jitter(node_id: &str, max_jitter_ms: u64) -> u64 {
    if max_jitter_ms == 0 {
        return 0;
    }

    let sum = node_id.bytes().map(u64::from).sum::<u64>();
    sum % max_jitter_ms
}
