use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::dimensions::Covariance;
use kindle_burn::tensor::Tensor;

define_tensor!(vis = pub, dim = 4);

fn main() {
    let a: Tensor4<Wgpu, WgpuBestAvailableDevice, 1, 2, 3, 4> =
        Tensor4::from_float_unchecked([[[[1.0; 4]; 3]; 2]; 1]);
    let b: Tensor4<Wgpu, WgpuBestAvailableDevice, 1, 2, 4, 5> =
        Tensor4::from_float_unchecked([[[[1.0; 5]; 4]; 2]; 1]);
    let mul = a.matmul(b);
}
