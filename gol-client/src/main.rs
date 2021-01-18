#[cfg(any(feature = "ascii"))]
use gol_client::demo::glider_gun_2d;

fn main() {
    #[cfg(not(any(feature = "ascii")))]
    panic!("No render engine backend specified. Specify \"--features [ascii, gl, vulkan, metal, dx11, dx12]\" at compile time to enable render endinges.");

    #[cfg(any(feature = "ascii"))]
    glider_gun_2d::run_demo(300, 3.0, 0.1);
}
