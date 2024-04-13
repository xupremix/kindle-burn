use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::dimensions::Covariance;
use kindle_burn::tensor::Tensor;

define_tensor!(vis = pub, dim = 2);

fn main() {
    let t = Tensor::<Wgpu, 1>::ones([10], &Default::default());
    let cov = t.cov(0, 0);
    println!("{:?}", cov.dims());

    let cov: Tensor2<Wgpu, WgpuBestAvailableDevice, 1, 2> =
        Tensor2::from_float_unchecked([[1.0; 2]; 1]);
    let cov = Covariance::<0>::cov(cov, 0);
    println!("{:?}", cov.dims());
}
