use kindle_burn::backend::{Fusion, Wgpu};
use kindle_burn::define_tensor;
use kindle_burn::device::KindleDevice;
use kindle_burn::device::{KindleFusionDevice, WgpuBestAvailableDevice};
use kindle_burn::tensor::Shape;

define_tensor!(vis = pub, dim = 3);

fn main() {
    let t_cpu: Tensor3<
        // 'static,
        Fusion<Wgpu>,
        KindleFusionDevice<WgpuBestAvailableDevice, Wgpu>,
        2,
        2,
        2,
        kindle_burn::tensor::Float,
    > = Tensor3 {
        tensor: kindle_burn::tensor::Tensor::<Fusion<Wgpu>, 3>::zeros(Shape::new([2; 3]), &{
            let d = KindleFusionDevice::<WgpuBestAvailableDevice, Wgpu>::to_device();
            println!("{:?}", d);
            d
        }),
        _device: std::marker::PhantomData,
    };

    // let t = kindle_burn::tensor::Tensor::<Autodiff<Wgpu>, 2>::zeros(
    //     Shape::new([2; 2]),
    //     &Default::default(),
    // );
    // let t = kindle_burn::tensor::Tensor::<Fusion<Candle>, 2>::zeros(
    //     Shape::new([2; 2]),
    //     &FusionDevice<'_, CandleCpuDevice, Candle>.into(),
    // );
    //
    // let t2 = t_cpu + t_gpu;
    // println!("{:?}", t2);
}
