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
        empty.slice::<0, 101, 0, 100, 0, 150, Range<0, 101>, Range<0, 100>, Range<0, 150>>();
    // println!("{:#?}", empty);
    // let new = empty.transpose();

    // let swapped = Swap::<1, 2>::swap_dims(empty);
    // println!("{:?}", swapped.shape());
}
