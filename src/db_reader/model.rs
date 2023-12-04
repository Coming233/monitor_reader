use std::{clone, collections::VecDeque};

#[derive(Debug, Clone)]
pub struct MonitorVec {
    pub len: usize,
    pub load_1: VecDeque<f32>,
    pub load_5: VecDeque<f32>,
    pub load_15: VecDeque<f32>,
    pub cpu_usage: VecDeque<f32>,
    pub memory_used: VecDeque<u64>,
    pub memory_free: VecDeque<u64>,
    pub swap_used: VecDeque<u64>,
    pub swap_free: VecDeque<u64>,
    pub disk_used: VecDeque<u64>,
    pub disk_read: VecDeque<u64>,
    pub disk_write: VecDeque<u64>,
    pub network_rx: VecDeque<u64>,
    pub network_tx: VecDeque<u64>,
}

impl MonitorVec {
    pub fn new(queue_len: usize) -> Self {
        let default_data_f32 = VecDeque::new();
        let default_data_u64 = VecDeque::new();
        Self {
            len: queue_len,
            load_1: default_data_f32.clone(),
            load_5: default_data_f32.clone(),
            load_15: default_data_f32.clone(),
            cpu_usage: default_data_f32.clone(),
            memory_used: default_data_u64.clone(),
            memory_free: default_data_u64.clone(),
            swap_used: default_data_u64.clone(),
            swap_free: default_data_u64.clone(),
            disk_used: default_data_u64.clone(),
            disk_read: default_data_u64.clone(),
            disk_write: default_data_u64.clone(),
            network_rx: default_data_u64.clone(),
            network_tx: default_data_u64.clone(),
        }
    }
}
