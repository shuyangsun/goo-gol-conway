#[derive(Clone)]
pub struct RendererBoardInfo<T> {
    board_shape: T,
    title: String,
    cur_iter: Option<usize>,
}

impl<T> RendererBoardInfo<T> {
    pub fn new(board_shape: T) -> Self {
        Self {
            board_shape,
            title: String::from(""),
            cur_iter: None,
        }
    }

    pub fn board_shape(&self) -> &T {
        &self.board_shape
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
