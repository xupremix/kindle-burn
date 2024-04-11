// use kindle_burn::backend::wgpu::Vulkan;
use kindle_burn::backend::Autodiff;
use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::KindleDevice;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::device::WgpuCpuDevice;

// use kindle_burn::dimensions::Swap;

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

    let my_first: Tensor3<Autodiff<Wgpu>, WgpuCpuDevice, 10, 20, 30, kindle_burn::tensor::Float> =
        Tensor3 {
            tensor: kindle_burn::tensor::Tensor::<Autodiff<Wgpu>, 3>::ones(
                [10, 20, 30],
                &<WgpuCpuDevice as KindleDevice<Autodiff<Wgpu>>>::to_device(),
                // &<WgpuBestAvailableDevice as KindleDevice<Autodiff<Wgpu>>>::to_device(),
                // &<WgpuBestAvailableDevice<Vulkan> as KindleDevice<Autodiff<Wgpu<Vulkan>>>>::to_device(),
            ),
            _device: std::marker::PhantomData,
        };
    let my_second: Tensor3<
        Autodiff<Wgpu>,
        WgpuCpuDevice,
        // WgpuBestAvailableDevice,
        // Autodiff<Wgpu<Vulkan>>,
        // WgpuBestAvailableDevice<Vulkan>,
        10,
        10,
        10,
        kindle_burn::tensor::Float,
    > = Tensor3 {
        tensor: kindle_burn::tensor::Tensor::<Autodiff<Wgpu>, 3>::ones(
            // tensor: kindle_burn::tensor::Tensor::<Autodiff<Wgpu<Vulkan>>, 3>::ones(
            [10, 10, 10],
            &<WgpuCpuDevice as KindleDevice<Autodiff<Wgpu>>>::to_device(),
            // &<WgpuBestAvailableDevice as KindleDevice<Autodiff<Wgpu>>>::to_device(),
            // &<WgpuBestAvailableDevice<Vulkan> as KindleDevice<Autodiff<Wgpu<Vulkan>>>>::to_device(),
        ),
        _device: std::marker::PhantomData,
    };

    let first = kindle_burn::tensor::Tensor::<Wgpu, 3>::ones([10, 20, 30], &Default::default());
    let second = kindle_burn::tensor::Tensor::<Wgpu, 3>::ones([10, 10, 10], &Default::default());

    let ris_first = first.slice_assign([0..10, 0..10, 0..10], second);
    println!("{:?}", ris_first.dims());
    let my_ris_first = my_first.slice_assign::<10, 10, 10>(my_second);
    println!("{:?}", my_ris_first.dims());
    // TODO: See in the macro definition of slice_assign for TensorN
    // it would become
    // let my_ris_first = my_first.slice_assign::<10, 10, 10>(my_second);
    // which seems more concise

    // let sliced = empty.slice::<5, 2, 0, 20, 0, 10>();

    // let replaced = empty.slice_assign::<10, 10, 10, 0, 10, 0, 10, 0, 10>(other);

    // let rang = a::<0, 11>();

    // println!("{:#?}", empty);
    // let new = empty.transpose();

    // let swapped = Swap::<1, 2>::swap_dims(empty);
    // println!("{:?}", swapped.shape());
}
