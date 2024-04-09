use kindle_burn::backend::wgpu::AutoGraphicsApi;
use kindle_burn::backend::Autodiff;
use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::KindleDevice;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::tensor::Shape;

define_tensor!(vis = pub, dim = 3);

fn main() {
    let t_gpu: Tensor3<
        Autodiff<Wgpu>,
        WgpuBestAvailableDevice,
        2,
        2,
        2,
        kindle_burn::tensor::Float,
    > = Tensor3 {
        tensor: kindle_burn::tensor::Tensor::<Autodiff<Wgpu>, 3>::zeros(
            Shape::new([2; 3]),
            &WgpuBestAvailableDevice::<AutoGraphicsApi, f32, i32>::to_device(),
        ),
        _device: std::marker::PhantomData,
    };

    // let t_gpu: Tensor3<Wgpu, WgpuBestAvailableDevice, 2, 2, 2, kindle_burn::tensor::Float> =
    //     Tensor3 {
    //         tensor: kindle_burn::tensor::Tensor::<Wgpu, 3>::zeros(
    //             Shape::new([2; 3]),
    //             &WgpuBestAvailableDevice::<AutoGraphicsApi, f32, i32>::to_device(),
    //         ),
    //         _device: std::marker::PhantomData,
    //     };

    // let t = kindle_burn::tensor::Tensor::<Autodiff<Wgpu>, 3>::zeros(
    //     Shape::new([2; 3]),
    //     &Default::default(),
    // );

    // let t_cpu: Tensor3<Candle, CandleCpuDevice, 2, 2, 2, kindle_burn::tensor::Float> = Tensor3 {
    //     tensor: kindle_burn::tensor::Tensor::<Candle, 3>::zeros(
    //         Shape::new([2; 3]),
    //         &CandleCpuDevice::to_device(),
    //     ),
    //     _device: std::marker::PhantomData,
    // };

    // let t_cpu: Tensor3<
    //     Fusion<Wgpu>,
    //     KindleFusionDevice<WgpuBestAvailableDevice, Wgpu>,
    //     2,
    //     2,
    //     2,
    //     kindle_burn::tensor::Float,
    // > = Tensor3 {
    //     tensor: kindle_burn::tensor::Tensor::<Fusion<Wgpu>, 3>::zeros(
    //         Shape::new([2; 3]),
    //         &KindleFusionDevice::<WgpuBestAvailableDevice, Wgpu>::to_device(),
    //     ),
    //     _device: std::marker::PhantomData,
    // };
}
