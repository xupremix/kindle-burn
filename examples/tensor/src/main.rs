use kindle_burn::backend::wgpu::Vulkan;
use kindle_burn::backend::Autodiff;
use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::dimensions::Swap;

define_tensor!(vis = pub, dim = 3);

fn main() {
    let empty = Tensor3::<
        Autodiff<Wgpu<Vulkan>>,
        WgpuBestAvailableDevice<Vulkan>,
        10,
        20,
        30,
        kindle_burn::tensor::Float,
    >::empty();

    let sliced = empty.slice::<5, 2, 1, 20, 0, 10>();

    // let sliced = empty.slice::<0, 10, 0, 10, 0, 15, Range<0, 10>, Range<0, 10>, Range<0, 15>>();

    // let rang = a::<0, 11>();

    // println!("{:#?}", empty);
    // let new = empty.transpose();

    // let swapped = Swap::<1, 2>::swap_dims(empty);
    // println!("{:?}", swapped.shape());
}
