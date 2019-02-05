extern crate sysinfo;

use sysinfo::{SystemExt, ProcessorExt};

pub struct System {
    system: sysinfo::System,
    pub cpu_usage_history: Vec<u64>,
    pub cpu_current_usage: u64,
    pub cpu_num_cores: usize,
    pub mem_total: u64,
    pub mem_free: u64,
    pub mem_used: u64,
    pub mem_usage_history: Vec<u64>,
    pub cpu_core_usages: Vec<u16>
}

impl System {
    pub fn new(system: sysinfo::System, initial_size: u16) -> System {
        let history_width = initial_size / 2;

        // Overall CPU usage
        let cpu_usage_history = vec![0; history_width as usize];
        let cpu_num_cores: usize = system.get_processor_list().len() - 1;

        // Memory usage
        let mem_total = system.get_total_memory();
        let mem_usage_history = vec![0; history_width as usize];

        System {
            system,
            cpu_usage_history,
            cpu_current_usage: 0,
            cpu_num_cores,
            mem_total,
            mem_free: 0,
            mem_used: 0,
            mem_usage_history,
            cpu_core_usages: vec![]
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_all();

        // Overall CPU usage
        self.cpu_current_usage = (self.system.get_processor_list()[0].get_cpu_usage() * 100.0).round() as u64;
        self.cpu_usage_history.push(self.cpu_current_usage);
        self.cpu_usage_history.remove(0);

        // Memory usage
        self.mem_used = self.system.get_used_memory();
        self.mem_free = self.system.get_free_memory();
        self.mem_usage_history.push(self.mem_used);
        self.mem_usage_history.remove(0);

        // CPU core usage
        self.cpu_core_usages = self.system.get_processor_list()
            .iter()
            .skip(1)
            .map(|p| (p.get_cpu_usage() * 100.0).round() as u16)
            .collect();
    }
}
