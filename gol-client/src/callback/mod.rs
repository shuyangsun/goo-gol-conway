pub mod delay;
pub mod pause;
pub mod terminate;

pub fn standard_control_callbacks<T, U, I>(
    delay_interval: std::time::Duration,
) -> (
    Vec<gol_core::BoardCallback<T, U, I>>,
    crate::util::keyboard_control::KeyboardControl,
)
where
    T: Send + Sync + Clone,
    U: Send + Sync + Clone,
    I: rayon::iter::ParallelIterator<Item = gol_core::IndexedDataOwned<U, T>>,
{
    use crate::util::keyboard_control::KeyboardControl;
    use delay::Delay;
    use gol_core::BoardCallback;
    use pause::Pause;
    use terminate::Terminate;

    let mut res = Vec::new();
    let keyboard_control = KeyboardControl::new();
    let delay = BoardCallback::WithoutStates(Box::new(Delay::new_with_ch_receiver(
        delay_interval,
        keyboard_control.get_receiver(),
    )));
    let pause =
        BoardCallback::WithoutStates(Box::new(Pause::new(false, keyboard_control.get_receiver())));
    let terminate =
        BoardCallback::WithoutStates(Box::new(Terminate::new(keyboard_control.get_receiver())));

    res.push(delay);
    res.push(pause);
    res.push(terminate);

    (res, keyboard_control)
}
