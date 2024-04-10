use kindle_burn::backend::wgpu::Vulkan;
use kindle_burn::backend::Autodiff;
use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::dimensions::ConstRange;
use kindle_burn::dimensions::Range;
// use kindle_burn::dimensions::Range;
use kindle_burn::dimensions::Swap;

define_tensor!(vis = pub, dim = 3);

fn main() {
    let empty = Tensor3::<
        Autodiff<Wgpu<Vulkan>>,
        WgpuBestAvailableDevice<Vulkan>,
        100,
        200,
        300,
        kindle_burn::tensor::Float,
    >::empty();

    let sliced =
        empty.slice::<Range<0, 100, 0, 100>, Range<0, 200, 0, 200>, Range<0, 300, 0, 300>>();
    // println!("{:#?}", empty);
    // let new = empty.transpose();

    // let swapped = Swap::<1, 2>::swap_dims(empty);
    // println!("{:?}", swapped.shape());
}
