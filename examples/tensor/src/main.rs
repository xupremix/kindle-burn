use kindle_burn::backend::Autodiff;
use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::KindleDevice;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::tensor::Shape;

define_tensor!(vis = pub, dim = 3);

fn main() {
    let empty = Tensor3::<
        Autodiff<Wgpu>,
        WgpuBestAvailableDevice,
        2,
        3,
        4,
        kindle_burn::tensor::Float,
    >::empty();
    let transposed = empty.transpose();
    println!("{:?}", transposed.shape());
}
