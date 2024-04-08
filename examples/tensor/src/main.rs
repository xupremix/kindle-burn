use kindle_burn::define_tensor;

define_tensor!(vis = pub(crate), dim = 2);

fn main() {
    // let t_gpu = kindle_burn::tensor::Tensor::<Wgpu, 2>::zeros(
    //     Shape::new([2, 2]),
    //     &WgpuDevice::BestAvailable,
    // );
    //
    // let t2 = t_cpu + t_gpu;
    // println!("{:?}", t2);
}
