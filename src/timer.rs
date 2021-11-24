use sdl2::TimerSubsystem;

use crate::constants::FRAME_TARGET_TIME;

pub struct Timer {
    fps: u8,
    interval: u32,
    last_tick: u32,
    last_second: u32,
    timer_subsystem: TimerSubsystem,
}

impl Timer {
    pub fn new(timer_subsystem: TimerSubsystem) -> Timer {
        let now = timer_subsystem.ticks();

        Timer {
            fps: 0,
            interval: FRAME_TARGET_TIME,
            last_tick: now,
            last_second: now,
            timer_subsystem: timer_subsystem,
        }
    }

    pub fn tick(&mut self) -> (bool, f32) {
        let now = self.timer_subsystem.ticks();
        let time_elapsed = now - self.last_tick;

        if now < self.last_tick + self.interval as u32 {
            self.timer_subsystem.delay(self.interval - time_elapsed);
            return (false, 0.0);
        }

        self.last_tick = now;
        self.fps += 1;
        if now - self.last_second > 1000 {
            // println!("FPS: {}", self.fps);
            self.fps = 0;
            self.last_second = now;
        }

        let mut delta = time_elapsed as f32 / 1000f32;
        if delta > 0.05 {
            delta = 0.05
        }
        return (true, delta);
    }
}
