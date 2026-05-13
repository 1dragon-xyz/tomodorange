mod audio;
mod config;

use cosmic::app::{Core, Task};
use cosmic::iced::widget::{button, column, text, container};
use cosmic::iced::{Alignment, Length, Subscription, time, window};
use audio::AudioEngine;
use config::Config;
use std::sync::Arc;

struct TomodOrange {
    core: Core,
    config: Config,
    remaining_secs: u32,
    state: TimerState,
    is_running: bool,
    audio: Arc<AudioEngine>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TimerState {
    Work,
    Break,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
    ToggleTimer,
}

impl cosmic::Application for TomodOrange {
    type Message = Message;
    type Flags = ();
    type Executor = cosmic::executor::Default;
    const APP_ID: &'static str = "com.michaels.tomodorange";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let config = Config::load();
        let app = Self {
            core,
            remaining_secs: config.work_minutes * 60,
            config,
            state: TimerState::Work,
            is_running: false,
            audio: Arc::new(AudioEngine::new().expect("Audio init failed")),
        };

        let task = if app.config.always_on_top {
            let window_id = app.core.main_window_id().unwrap();
            window::set_level(window_id, window::Level::AlwaysOnTop).into()
        } else {
            Task::none()
        };

        (app, task)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                    if self.is_running && self.remaining_secs > 0 {
                        self.remaining_secs -= 1;
                        
                        if self.state == TimerState::Work && self.config.sound_enabled {
                            self.audio.play_tick();
                        }

                    if self.remaining_secs == 0 {
                        self.switch_state();
                    }
                }
            }
            Message::ToggleTimer => {
                self.is_running = !self.is_running;
                if !self.is_running {
                    self.audio.stop_break_loop();
                } else if self.state == TimerState::Break && self.config.sound_enabled {
                    self.audio.start_break_loop();
                }
            }
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.is_running {
            time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> cosmic::Element<'_, Message> {
        let time_str = format!("{:02}:{:02}", self.remaining_secs / 60, self.remaining_secs % 60);

        let content = column![
            text(time_str).size(60),
            button(if self.is_running { "Pause" } else { "Start" })
                .on_press(Message::ToggleTimer)
        ]
        .spacing(10)
        .align_x(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

impl TomodOrange {
    fn switch_state(&mut self) {
        // Play Gong on every transition
        if self.config.sound_enabled {
            self.audio.play_gong();
        }
        
        if self.state == TimerState::Work {
            self.state = TimerState::Break;
            self.remaining_secs = self.config.break_minutes * 60;
            if self.config.sound_enabled {
                self.audio.start_break_loop();
            }
        } else {
            self.state = TimerState::Work;
            self.remaining_secs = self.config.work_minutes * 60;
            self.audio.stop_break_loop();
        }
        self.is_running = true;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cosmic::app::run::<TomodOrange>(cosmic::app::Settings::default(), ())?;
    Ok(())
}
