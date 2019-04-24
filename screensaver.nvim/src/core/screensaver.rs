use neovim_lib::{neovim_api::Buffer, Neovim};

pub struct Screensaver {
    started: bool,
    orig: Vec<String>,
}

impl Screensaver {
    pub fn new() -> Self {
        Self {
            started: false,
            orig: vec![],
        }
    }

    fn hide_str(&self, s: &String) -> String {
        let st = s.trim_start();
        let pad = str::repeat(" ", s.len() - st.len()).to_string();

        format!("{}{}", pad, str::repeat("*", st.len()))
    }

    pub fn stop(&mut self, nvim: &mut Neovim, buf: Buffer) {
        if self.started {
            self.started = false;

            buf.set_lines(nvim, 0, self.orig.len() as i64, false, self.orig.clone())
                .unwrap();
        }
    }

    pub fn start(&mut self, nvim: &mut Neovim, buf: Buffer) {
        if !self.started {
            self.started = true;

            let c = buf.line_count(nvim).unwrap();
            self.orig = buf.get_lines(nvim, 0, c, false).unwrap();

            let mut v = vec![];
            for s in self.orig.iter() {
                v.push(self.hide_str(s));
            }
            buf.set_lines(nvim, 0, v.len() as i64, false, v).unwrap();
        }
    }
}
