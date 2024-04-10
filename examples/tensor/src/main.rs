use kindle_burn::backend::wgpu::Vulkan;
use kindle_burn::backend::Autodiff;
use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::KindleDevice;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::dimensions::Swap;

define_tensor!(vis = pub, dim = 3);

fn main() {
    // let empty = Tensor3::<
    //     Autodiff<Wgpu<Vulkan>>,
    //     WgpuBestAvailableDevice<Vulkan>,
    //     10,
    //     20,
    //     30,
    //     kindle_burn::tensor::Float,
    // >::empty();
    let empty: Tensor3<
        Autodiff<Wgpu<Vulkan>>,
        WgpuBestAvailableDevice<Vulkan>,
        10,
        20,
        30,
        kindle_burn::tensor::Float,
    > = Tensor3 {
        tensor: kindle_burn::tensor::Tensor::ones([10, 20, 30], &Default::default()),
        _device: std::marker::PhantomData,
    };

    let other: Tensor3<
        Autodiff<Wgpu<Vulkan>>,
        WgpuBestAvailableDevice<Vulkan>,
        10,
        10,
        10,
        kindle_burn::tensor::Float,
    > = Tensor3 {
        tensor: kindle_burn::tensor::Tensor::ones([10, 20, 30], &Default::default()),
        _device: std::marker::PhantomData,
    };

    // let sliced = empty.slice::<5, 2, 0, 20, 0, 10>();

    let replaced = empty.slice_assign::<10, 10, 10, 0, 10, 0, 10, 0, 10>(other);

    // let rang = a::<0, 11>();

    // println!("{:#?}", empty);
    // let new = empty.transpose();

    // let swapped = Swap::<1, 2>::swap_dims(empty);
    // println!("{:?}", swapped.shape());
}
