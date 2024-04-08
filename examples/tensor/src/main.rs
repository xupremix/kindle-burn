use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::WgpuCpuDevice;

define_tensor!(vis = pub, dim = 2);

fn main() {
    // let t_gpu = kindle_burn::tensor::Tensor::<Wgpu, 2>::zeros(
    //     Shape::new([2, 2]),
    //     &WgpuDevice::BestAvailable,
    // );
    //
    // let t2 = t_cpu + t_gpu;
    // println!("{:?}", t2);
}
