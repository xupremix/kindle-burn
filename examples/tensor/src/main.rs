use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::tensor::Tensor;

define_tensor!(vis = pub, dim = 3);
define_tensor!(vis = pub, dim = 2);

fn main() {
    let t = Tensor::<Wgpu, 3>::ones([10, 11, 12], &Default::default());
    let t2 = Tensor::<Wgpu, 3>::ones([10, 12, 13], &Default::default());

    let out = t.matmul(t2);
    println!("{:?}", out.dims());

    let t: Tensor3<Wgpu, WgpuBestAvailableDevice, 10, 11, 12> = Tensor3::empty();
    let t2: Tensor3<Wgpu, WgpuBestAvailableDevice, 10, 12, 13> = Tensor3::empty();
    let out = t.matmul(t2);
    println!("{:?}", out.dims());
}
