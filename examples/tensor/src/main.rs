use kindle_burn::backend::Wgpu;
use kindle_burn::tensor::Shape;
use kindle_burn::{backend::wgpu::WgpuDevice, define_tensor};

define_tensor!(vis = pub(crate), dim = 2);

fn main() {
    // let t_cpu = kindle_burn::tensor::Tensor::<Wgpu, 2>::zeros(Shape::new([2, 2]), &WgpuDevice::Cpu);
    // let t_gpu = kindle_burn::tensor::Tensor::<Wgpu, 2>::zeros(
    //     Shape::new([2, 2]),
    //     &WgpuDevice::BestAvailable,
    // );
    //
    // let t2 = t_cpu + t_gpu;
    // println!("{:?}", t2);
}
