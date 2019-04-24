use neovim_lib::{Neovim, NeovimApi, Session};

use super::screensaver::Screensaver;

pub struct EventHandler {
    nvim: Neovim,
    screensaver: Screensaver,
}

impl EventHandler {
    pub fn new() -> Self {
        match Session::new_parent() {
            Ok(r) => Self {
                nvim: Neovim::new(r),
                screensaver: Screensaver::new(),
            },
            Err(e) => panic!(e),
        }
    }

    pub fn recv(&mut self) {
        for (evt, _) in self.nvim.session.start_event_loop_channel() {
            if let Ok(buf) = self.nvim.get_current_buf() {
                if evt == "stop" {
                    self.screensaver.stop(&mut self.nvim, buf);
                } else if evt == "start" {
                    self.screensaver.start(&mut self.nvim, buf);
                }
            }
        }
    }
}
