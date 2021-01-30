#[derive(Clone)]
pub struct RendererBoardInfo<T> {
    board_size: T,
    title: String,
    cur_iter: Option<usize>,
}

impl<T> RendererBoardInfo<T> {
    pub fn new(board_size: T) -> Self {
        Self {
            board_size,
            title: String::from(""),
            cur_iter: None,
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
        self.cur_iter
    }

    pub fn set_iter_count(&mut self, iter_count: usize) {
        self.cur_iter = Some(iter_count);
    }
}
