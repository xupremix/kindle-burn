use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::KindleDevice;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::dimensions::Repeat;
use kindle_burn::dimensions::Swap;

define_tensor!(vis = pub, dim = 3);

fn main() {
    let t = kindle_burn::tensor::Tensor::<Wgpu, 3>::ones([10, 1, 30], &Default::default());
    let _t0 = kindle_burn::tensor::Tensor::<Wgpu, 3>::zeros([10, 20, 30], &Default::default());

    let empty: Tensor3<Wgpu, WgpuBestAvailableDevice, 10, 1, 30> = Tensor3::empty();
    // let out = zeros.slice::<0, 10, 0, 20, 0, 30>();

    let out = t.repeat(1, 100);
    let my_out = Repeat::<1, 100>::repeat(empty);
    // let other = Swap::<1, 2>::swap_dims(empty);
    println!("{:?}", out.dims());
    println!("{:?}", my_out.dims());
    // let out = Tensor::cat(vec![t, t0], 0);
}
