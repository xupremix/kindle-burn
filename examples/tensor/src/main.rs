use kindle_burn::backend::wgpu::{JitBackend, WgpuRuntime};
use kindle_burn::backend::{Autodiff, NdArray, Wgpu};
use kindle_burn::define_tensor;
use kindle_burn::device::{KindleFusionDevice, NdArrayCpuDevice, WgpuBestAvailableDevice};
use kindle_burn::fusion::Fusion;

define_tensor!(vis = pub, dim = 4);

fn main() {
    // Wgpu backend
    let t1: Tensor4<Wgpu, WgpuBestAvailableDevice, 1, 2, 3, 4> = Tensor4::zeros();
    let t2: Tensor4<Wgpu, WgpuBestAvailableDevice, 1, 2, 4, 5> = Tensor4::ones();
    let t3 = t1.matmul(t2);
    println!("{:?}", t3.shape());

    // Ndarray backend
    let t1 = Tensor4::<NdArray, NdArrayCpuDevice, 1, 2, 3, 4>::zeros();
    let t2 = Tensor4::<NdArray, NdArrayCpuDevice, 1, 2, 4, 5>::zeros();
    let t3 = t1.matmul(t2);
    println!("{:?}", t3.shape());

    // Explicit fusion device with Wgpu backend
    let t1 = Tensor4::<
        Fusion<JitBackend<WgpuRuntime, f32, i32>>,
        KindleFusionDevice<WgpuBestAvailableDevice>,
        1,
        2,
        3,
        4,
    >::zeros();
    let t2 = Tensor4::<
        Fusion<JitBackend<WgpuRuntime, f32, i32>>,
        KindleFusionDevice<WgpuBestAvailableDevice>,
        1,
        2,
        4,
        5,
    >::zeros();
    let t3 = t1.matmul(t2);
    println!("{:?}", t3.shape());

    // Autodiff + Fusion device with Wgpu runtime
    let t1 = Tensor4::<
        Autodiff<Fusion<JitBackend<WgpuRuntime, f32, i32>>>,
        KindleFusionDevice<WgpuBestAvailableDevice>,
        1,
        2,
        3,
        4,
    >::zeros();
    let t2 = Tensor4::<
        Autodiff<Fusion<JitBackend<WgpuRuntime, f32, i32>>>,
        KindleFusionDevice<WgpuBestAvailableDevice>,
        1,
        2,
        4,
        5,
    >::zeros();
    let t3 = t1.matmul(t2);
    println!("{:?}", t3.shape());
}
