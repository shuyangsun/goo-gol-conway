use std::time::Instant;

pub struct FPSCounter {
    max_buffer_size: usize,
    buffer: Vec<f32>,
    cur_avg: f32,
    last_execution: Option<Instant>,
}

impl Default for FPSCounter {
    fn default() -> Self {
        Self::new(10)
    }
}

impl FPSCounter {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            max_buffer_size: buffer_size,
            buffer: Vec::with_capacity(buffer_size),
            cur_avg: 0.0,
            last_execution: None,
        }
    }

    pub fn lapse(&mut self) {
        let new_ts = Instant::now();
        let last_ts = self.last_execution;
        self.last_execution = Some(new_ts);

        if last_ts.is_none() {
            return;
        }
        let last_ts = last_ts.unwrap();

        let cur_len = self.buffer.len();
        let mut accum = if self.cur_avg == 0.0 {
            0.0
        } else {
            1.0 / self.cur_avg * cur_len as f32
        };
        let cur_dur = (new_ts - last_ts).as_secs_f32();
        accum += cur_dur;
        self.buffer.push(cur_dur);
        if self.buffer.len() > self.max_buffer_size {
            let pop = self.buffer.remove(0);
            accum -= pop;
        }
        self.cur_avg = self.buffer.len() as f32 / accum;
    }

    pub fn fps(&self) -> f32 {
        self.cur_avg
    }
}
