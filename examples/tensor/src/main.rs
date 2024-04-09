use kindle_burn::backend::{Fusion, Wgpu};
use kindle_burn::define_tensor;
use kindle_burn::device::{KindleFusionDevice, WgpuBestAvailableDevice, WgpuCpuDevice};
// use kindle_burn::device::{CandleCpuDevice, FusionDevice};
use kindle_burn::device::KindleDevice;
use kindle_burn::tensor::Shape;

define_tensor!(vis = pub, dim = 3);

fn main() {
    let t_cpu = kindle_burn::tensor::Tensor::<Fusion<Wgpu>, 2>::zeros(Shape::new([2; 2]), &{
        let d = KindleFusionDevice::<WgpuBestAvailableDevice, Wgpu>::to_device();
        println!("{:?}", d);
        d
    });

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
