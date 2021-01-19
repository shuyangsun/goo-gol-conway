extern crate ncurses;

use crate::TextRendererGrid2D;
use gol_core::{BoardCallback, GridPoint2D, IndexedDataOwned};
use num_traits::{CheckedSub, FromPrimitive, ToPrimitive};
use rayon::prelude::*;

pub struct TextRendererGrid2DShadow {
    text_renderer: TextRendererGrid2D,
}

impl<T, U, I> BoardCallback<T, GridPoint2D<U>, I> for TextRendererGrid2DShadow
where
    T: Send + Sync + Clone + std::convert::Into<char>,
    U: Send + Sync + Clone + Ord + CheckedSub + ToPrimitive + FromPrimitive,
    I: ParallelIterator<Item = IndexedDataOwned<GridPoint2D<U>, T>>,
{
    fn setup(&mut self) {
        <TextRendererGrid2D as BoardCallback<T, GridPoint2D<U>, I>>::setup(&mut self.text_renderer);
    }

    fn cleanup(&mut self) {
        <TextRendererGrid2D as BoardCallback<T, GridPoint2D<U>, I>>::cleanup(
            &mut self.text_renderer,
        );
    }

    fn execute(&mut self, states: I) {
        self.text_renderer.execute(states);
        // TODO: implement board mapper (for dimensionality reduction.)
        // let (x_min, x_max, y_min, y_max) = self.text_renderer.window_dim.unwrap();
        // let win_width = x_max.checked_sub(x_min).unwrap().to_i32().unwrap() + 4;
        // let win_height = y_max.checked_sub(y_min).unwrap().to_i32().unwrap() + 4;
        // let start_y = (self.text_renderer.screen_dim.0 - win_height) / 2;
        // let start_x = (self.text_renderer.screen_dim.1 - win_width) / 2;
        // let win_bottom = create_win(self.text_renderer.screen_dim.0 - 5, start_x, 5, win_width);
        // let win_left = create_win(start_y, start_x - 6, win_height, 5);
    }
}

impl TextRendererGrid2DShadow {
    pub fn new() -> Self {
        Self::new_with_title(String::from(""))
    }

    pub fn new_with_title(title: String) -> Self {
        Self {
            text_renderer: TextRendererGrid2D::new_with_title(title),
        }
    }
}
