pub mod delay;
pub mod keyboard_control;
pub mod pause;
pub mod terminate;

pub fn standard_control_callbacks<T, U, I>(
    delay_interval: std::time::Duration,
) -> (
    Vec<Box<dyn crate::BoardCallback<T, U, I>>>,
    crossbeam_channel::Sender<char>,
    crossbeam_channel::Receiver<char>,
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
    let keyboard_control = Box::new(KeyboardControl::new());
    let (tx, rx) = keyboard_control.get_channel();
    let delay = Box::new(Delay::new_with_ch_receiver(
        delay_interval,
        receiver.clone(),
    ));
    let pause = Box::new(Pause::new(receiver.clone()));
    let terminate = Box::new(Terminate::new(receiver.clone()));

    res.push(keyboard_control as Box<dyn crate::BoardCallback<T, U, I>>);
    res.push(delay as Box<dyn crate::BoardCallback<T, U, I>>);
    res.push(pause as Box<dyn crate::BoardCallback<T, U, I>>);
    res.push(terminate as Box<dyn crate::BoardCallback<T, U, I>>);

    (res, receiver)
}
