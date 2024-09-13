use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::tensor::Tensor;

define_tensor!(vis = pub, dim = 4);

fn main() {
    let t: Tensor<Wgpu, 3> = Tensor::ones([1, 2, 3], &Default::default());
    let t = t.repeat(&[3, 2, 4]);
    println!("Shape: {:?}", t.shape());

    let t1: Tensor4<Wgpu, WgpuBestAvailableDevice, 1, 2, 3, 4> = Tensor4::zeros();
    let t2: Tensor4<Wgpu, WgpuBestAvailableDevice, 1, 2, 4, 5> = Tensor4::ones();
    let t3 = t1.matmul(t2);
    println!("Shape: {:?}", t3.shape());
}
