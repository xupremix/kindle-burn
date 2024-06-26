use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::dimensions::Covariance;

define_tensor!(vis = pub, dim = 4);

fn main() {
    let a: Tensor4<Wgpu, WgpuBestAvailableDevice, 1, 2, 3, 4> = Tensor4::zeros();
    let a2: Tensor4<Wgpu, WgpuBestAvailableDevice, 1, 2, 3, 4> = Tensor4::ones();
    let a3 = a.add(a2);
    println!("{:?}", a3.to_data());
}
