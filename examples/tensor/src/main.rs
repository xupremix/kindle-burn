use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::KindleDevice;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::dimensions::Repeat;
use kindle_burn::dimensions::Swap;

define_tensor!(vis = pub, dim = 3);

fn main() {
    let t = kindle_burn::tensor::Tensor::<Wgpu, 3>::ones([10, 20, 30], &Default::default());
    let t0 = kindle_burn::tensor::Tensor::<Wgpu, 3>::zeros([10, 20, 30], &Default::default());
    let t1 = kindle_burn::tensor::Tensor::<Wgpu, 3>::zeros([10, 19, 30], &Default::default());

    let e: Tensor3<Wgpu, WgpuBestAvailableDevice, 10, 20, 30> = Tensor3::empty();
    let e1: Tensor3<Wgpu, WgpuBestAvailableDevice, 10, 20, 30> = Tensor3::empty();
    let e2: Tensor3<Wgpu, WgpuBestAvailableDevice, 10, 20, 30> = Tensor3::empty();

    // let other = Swap::<1, 2>::swap_dims(empty);

    let out = kindle_burn::tensor::Tensor::cat(vec![t, t0, t1], 1);
    println!("{:?}", out.dims());

    let my_out =
        Tensor3::<Wgpu, WgpuBestAvailableDevice, 10, 20, 30>::cat_unchecked::<1>(&[e, e1, e2]);
    // let my_out = kindle_burn::dimensions::Cat::<0, 3, 60>::cat(&[e, e1, e2]);

    // println!("{:?}", my_out.dims());
}
