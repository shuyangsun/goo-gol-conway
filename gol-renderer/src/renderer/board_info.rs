use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct RendererBoardInfo<T> {
    board_size: T,
    title: String,
    cur_iter: Arc<Mutex<Option<usize>>>,
}

impl<T> RendererBoardInfo<T> {
    pub fn new(board_size: T) -> Self {
        Self {
            board_size,
            title: String::from(""),
            cur_iter: Arc::new(Mutex::new(None)),
        }
    }

    pub fn board_size(&self) -> &T {
        &self.board_size
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn iter_count(&self) -> Option<usize> {
        *self.cur_iter.lock().unwrap()
    }

    pub fn set_iter_count(&self, iter_count: usize) {
        *self.cur_iter.lock().unwrap() = Some(iter_count);
    }
}
