use tuikit::attr::*;
use tuikit::term::{Term, TermHeight};

pub struct Window {
    term: Term,
    title_attr: Attr,
    line_attr: Attr,
    title: String,
    buf: Vec<String>,
}

impl Window {
    pub fn new(h: usize) -> Self {
        let term = Term::with_height(TermHeight::Fixed(h)).unwrap();
        let line_attr = Attr {
            fg: Color::LIGHT_BLACK,
            ..Attr::default()
        };

        let title_attr = Attr {
            fg: Color::MAGENTA,
            ..Attr::default()
        };

        Self {
            line_attr,
            title_attr,
            title: String::new(),
            buf: Vec::with_capacity(h),
            term,
        }
    }

    pub fn set_title(&mut self, s: String) {
        self.title = s;
    }

    pub fn push_line(&mut self, s: String) {
        if self.buf.len() == self.buf.capacity() {
            self.buf.drain(0..1);
        }

        self.buf.push(s);
    }

    pub fn present(&self) {
        let _ = self.term.clear();

        for (i, l) in self.buf.iter().enumerate() {
            let _ = self.term.print_with_attr(i, 0, l, self.line_attr);
        }

        let _ = self
            .term
            .print_with_attr(0, 0, self.title.as_str(), self.title_attr);

        let _ = self.term.present();
    }
}
