use std::mem;
use std::time::Instant;

trait Runnable {
    type TResult;
    fn run(&self) -> Self::TResult;
}

trait Facility<TRunnable: Runnable> {
    fn on_start(&mut self);
    fn on_end(&mut self);
    fn run(&mut self, r: TRunnable) -> TRunnable::TResult {
        self.on_start();
        let result = r.run();
        self.on_end();
        result
    }
}

pub struct TimerFacility {
    start_time: Instant,
    end_time: Instant
}

impl<TRunnable: Runnable> Facility<TRunnable> for TimerFacility {
    fn on_start(&mut self) {
        self.start_time = Instant::now();
    }

    fn on_end(&mut self) {
        self.end_time = Instant::now();
    }
}
