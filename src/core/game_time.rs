use std::time::{Duration, Instant};

pub struct GameTime
{
    start_time: Instant,
    last_update: Instant,
    frame_count :u32,
    fps: f32,
    elapsed_time: Duration,
    delta_time: Duration,
    target_fps : Option<u32>,
}

impl GameTime
{
    pub fn new() -> Self
    {
        let now = Instant::now();
        GameTime
        {
            start_time: now,
            last_update: now,
            frame_count: 0,
            fps: 0.0,
            elapsed_time: Duration::ZERO,
            delta_time: Duration::ZERO,
            target_fps : None,
        }
    }
    pub fn set_target_fps(&mut self, fps: u32) {
        self.target_fps = Some(fps);
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_update);
        self.elapsed_time = now.duration_since(self.start_time);
        self.last_update = now;

        self.frame_count += 1;
        if self.elapsed_time.as_secs_f32() >= 1.0 {
            self.fps = self.frame_count as f32;
            self.frame_count = 0;
            self.start_time = now;
        }
    }
    pub fn sleep_to_sync(&self) {
        if let Some(target_fps) = self.target_fps {
            let target_frame_time = Duration::from_secs_f32(1.0 / target_fps as f32);
            if self.delta_time < target_frame_time {
                std::thread::sleep(target_frame_time - self.delta_time);
            }
        }
    }
    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    pub fn elapsed_time(&self) -> Duration {
        self.elapsed_time
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }
}