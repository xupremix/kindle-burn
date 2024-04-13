use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::tensor::{Bool, Tensor};

define_tensor!(vis = pub, dim = 3);

fn main() {
    let t: Tensor3<Wgpu, WgpuBestAvailableDevice, 10, 20, 30> =
        Tensor3::from_float_unchecked([[[1.0; 30]; 20]; 10]);
    let t = t.into_full_precision();
}
