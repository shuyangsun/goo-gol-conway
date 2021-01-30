mod delay;
mod pause;
mod terminate;

pub fn standard_control_callbacks<T, U, I>(
    delay_interval: std::time::Duration,
) -> (
    Vec<gol_core::BoardCallback<T, U, I>>,
    gol_renderer::renderer::keyboard_control::KeyboardControl,
)
where
    T: Send + Sync + Clone,
    U: Send + Sync + Clone,
    I: rayon::iter::ParallelIterator<Item = gol_core::IndexedDataOwned<U, T>>,
{
    use delay::Delay;
    use gol_core::BoardCallback;
    use gol_renderer::renderer::keyboard_control::KeyboardControl;
    use pause::Pause;
    use terminate::Terminate;

    let mut res = Vec::new();
    let keyboard_control = KeyboardControl::new();
    let delay = BoardCallback::WithoutStates(Box::new(
        Delay::new(delay_interval).with_ch_receiver(keyboard_control.clone_receive_only()),
    ));
    let pause = BoardCallback::WithoutStates(Box::new(Pause::new(
        false,
        keyboard_control.clone_receive_only(),
    )));
    let terminate = BoardCallback::WithoutStates(Box::new(Terminate::new(
        keyboard_control.clone_receive_only(),
    )));

    res.push(delay);
    res.push(pause);
    res.push(terminate);

    (res, keyboard_control)
}
