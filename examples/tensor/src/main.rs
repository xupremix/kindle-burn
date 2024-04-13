use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::tensor::{Bool, Tensor};

define_tensor!(vis = pub, dim = 3);

fn main() {
    let t = Tensor::<Wgpu, 3>::ones([3, 2, 2], &Default::default());
    let var = t.var(0);
    println!("{:?}", var.dims());
}
