// macro_rules! impl_autodiff {
//     ($device:ident, $device_variant:ident $(,$n:ident)?) => {
//         impl<
//             $(const $n: usize,)?
//             GraphicsApi,
//             FloatElement,
//             IntElement,
//             Backend: crate::tensor::backend::AutodiffBackend,
//         >   crate::device::KindleDevice<
//             Backend
//         > for crate::device::$device <$($n,)? GraphicsApi, FloatElement, IntElement>
//         where
//             GraphicsApi: crate::backend::wgpu::GraphicsApi,
//             FloatElement: crate::backend::wgpu::FloatElement,
//             IntElement: crate::backend::wgpu::IntElement,
//         {
//             fn to_device() -> crate::backend::wgpu::WgpuDevice {
//                 crate::backend::wgpu::WgpuDevice::$device_variant $(($n))?
//             }
//         }
//     };
// }
//
// impl_autodiff!(WgpuBestAvailableDevice, BestAvailable);
// impl_autodiff!(WgpuIntegratedGpuDevice, IntegratedGpu, N);
// impl_autodiff!(WgpuVirtualGpuDevice, VirtualGpu, N);
// impl_autodiff!(WgpuCpuDevice, Cpu);
