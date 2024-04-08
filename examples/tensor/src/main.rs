use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::WgpuCpuDevice;
use kindle_burn::tensor::Shape;

define_tensor!(vis = pub, dim = 3);
// impl_transition!(unsqueeze, 2 -> 3);
// impl_transition!(squeeze, 3 -> 2);

fn main() {
    let k: Tensor3 = todo!();
    let t_gpu = kindle_burn::tensor::Tensor::<Wgpu, 0>::zeros(Shape::new([]), &Default::default());
    //
    // let t2 = t_cpu + t_gpu;
    // println!("{:?}", t2);
}
