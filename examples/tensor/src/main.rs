use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::tensor::Shape;

define_tensor!(vis = pub, dim = 3);

fn main() {
    // let t = kindle_burn::tensor::Tensor::<Autodiff<Wgpu>, 2>::zeros(
    //     Shape::new([2; 2]),
    //     &Default::default(),
    // );
    // let t = kindle_burn::tensor::Tensor::<Wgpu, 2>::zeros(Shape::new([2; 2]), &wgpu_device.into());
    //
    // let t2 = t_cpu + t_gpu;
    // println!("{:?}", t2);
}
