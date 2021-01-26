pub mod delay;
pub mod keyboard_control;
pub mod model_binary_states;
pub mod model_states;
pub mod pause;
pub mod terminate;

pub fn standard_control_callbacks<T, U, I>(
    delay_interval: std::time::Duration,
) -> (
    Vec<Box<dyn crate::BoardCallback<T, U, I>>>,
    crate::callback::keyboard_control::KeyboardControl,
)
where
    T: Send + Sync + Clone,
    U: Send + Sync + Clone,
    I: rayon::iter::ParallelIterator<Item = crate::IndexedDataOwned<U, T>>,
{
    use delay::Delay;
    use keyboard_control::KeyboardControl;
    use pause::Pause;
    use terminate::Terminate;

    let mut res = Vec::new();
    let keyboard_control = KeyboardControl::new();
    let delay = Box::new(Delay::new_with_ch_receiver(
        delay_interval,
        keyboard_control.get_receiver(),
    ));
    let pause = Box::new(Pause::new(false, keyboard_control.get_receiver()));
    let terminate = Box::new(Terminate::new(keyboard_control.get_receiver()));

    res.push(delay as Box<dyn crate::BoardCallback<T, U, I>>);
    res.push(pause as Box<dyn crate::BoardCallback<T, U, I>>);
    res.push(terminate as Box<dyn crate::BoardCallback<T, U, I>>);

    (res, keyboard_control)
}
