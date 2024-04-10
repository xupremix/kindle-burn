use kindle_burn::backend::wgpu::WebGpu;
use kindle_burn::backend::Autodiff;
use kindle_burn::backend::Candle;
use kindle_burn::backend::Fusion;
use kindle_burn::backend::NdArray;
use kindle_burn::backend::Wgpu;
use kindle_burn::define_tensor;
use kindle_burn::device::CandleCpuDevice;
use kindle_burn::device::CandleCudaDevice;
use kindle_burn::device::KindleDevice;
use kindle_burn::device::KindleFusionDevice;
use kindle_burn::device::NdArrayCpuDevice;
use kindle_burn::device::WgpuBestAvailableDevice;

define_tensor!(vis = pub, dim = 3);

fn main() {
    let empty = Tensor3::<
        Autodiff<Fusion<Wgpu>>,
        // WgpuBestAvailableDevice,
        KindleFusionDevice<WgpuBestAvailableDevice, Wgpu>,
        2,
        3,
        4,
        kindle_burn::tensor::Float,
    >::empty();
    let grads = empty.backward();
    let transposed = empty.transpose();
    println!("{:?}", transposed.shape());
}
