// use kindle_burn::backend::wgpu::Vulkan;
use kindle_burn::backend::Autodiff;
use kindle_burn::backend::Fusion;
use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::CandleCpuDevice;
use kindle_burn::device::KindleDevice;
use kindle_burn::device::KindleFusionDevice;
use kindle_burn::device::WgpuBestAvailableDevice;
use kindle_burn::device::WgpuCpuDevice;

// use kindle_burn::dimensions::Swap;

define_tensor!(vis = pub, dim = 3);

fn main() {
    // let t: Tensor3<Wgpu, WgpuBestAvailableDevice, 10, 20, 30> = Tensor3::empty();
    // let d = t.device();
    // let t2 = t.to_device::<WgpuCpuDevice>();
    let t: Tensor3<Wgpu, WgpuBestAvailableDevice, 10, 20, 30> =
        Tensor3::from_data_unchecked([[[1.0; 30]; 20]; 10]);
    let t2: Tensor3<Wgpu, WgpuBestAvailableDevice, 20, 20, 30> =
        Tensor3::from_data_unchecked([[[1.0; 30]; 20]; 20]);
    let out = t.equal(t2);
}
