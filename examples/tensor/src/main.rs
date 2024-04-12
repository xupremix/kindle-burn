use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::WgpuBestAvailableDevice;

define_tensor!(vis = pub, dim = 3);

fn main() {
    let t = kindle_burn::tensor::Tensor::<Wgpu, 3>::one_hot(3, 5, &Default::default());
    println!("Dims: {:?}", t.dims());
    println!("Data: {}", t.to_data());

    let my_t: Tensor3<Wgpu, WgpuBestAvailableDevice, 1, 1, 5> = Tensor3::one_hot::<3>();
    println!("Dims: {:?}", my_t.dims());
    println!("Data: {}", my_t.to_data());
}
