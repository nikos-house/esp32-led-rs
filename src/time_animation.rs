use crate::animation::AnimationRunner;
use std::time::Duration;

#[allow(dead_code)]
pub fn linear(t: f32) -> f32 {
    t
}

#[allow(dead_code)]
pub fn ease_in(t: f32) -> f32 {
    t * t
}

#[allow(dead_code)]
pub fn ease_out(t: f32) -> f32 {
    1.0 - ease_in(1.0 - t)
}

#[allow(dead_code)]
pub fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        ease_in(t * 2.0) / 2.0
    } else {
        ease_out((t - 0.5) * 2.0) / 2.0 + 0.5
    }
}

#[derive(Clone)]
pub struct TimeAnimationRunner {
    duration: Duration,
    looping: bool,
    loop_start_time: std::time::Instant,
    timing_fn: fn(f32) -> f32,
}

impl TimeAnimationRunner {
    #[allow(dead_code)]
    pub fn new(duration: Duration, looping: bool, timing_fn: fn(f32) -> f32) -> Self {
        Self {
            duration,
            looping,
            loop_start_time: std::time::Instant::now(),
            timing_fn,
        }
    }
}

impl AnimationRunner for TimeAnimationRunner {
    #[allow(dead_code)]
    fn get_progress(&mut self) -> f32 {
        let elapsed = std::time::Instant::now() - self.loop_start_time;
        if elapsed >= self.duration {
            if self.looping {
                let elapsed_time_since_last_loop = elapsed.as_millis() % self.duration.as_millis();
                self.loop_start_time = std::time::Instant::now()
                    - Duration::from_millis(elapsed_time_since_last_loop as u64);
                (self.timing_fn)(elapsed.as_millis() as f32 / self.duration.as_millis() as f32)
            } else {
                (self.timing_fn)(1.0)
            }
        } else {
            (self.timing_fn)(elapsed.as_millis() as f32 / self.duration.as_millis() as f32)
        }
    }
}
