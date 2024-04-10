macro_rules! impl_device {
    ($device:ident, $device_variant:ident $(,$n:ident)?) => {
        impl<
            $(const $n: usize,)?
            GraphicsApi,
            FloatElement,
            IntElement,
        >   crate::device::KindleDevice<
            crate::backend::Autodiff<
                crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>,
            >
        > for crate::device::$device <$($n,)? GraphicsApi, FloatElement, IntElement>
        where
            GraphicsApi: crate::backend::wgpu::GraphicsApi,
            FloatElement: crate::backend::wgpu::FloatElement,
            IntElement: crate::backend::wgpu::IntElement,
        {
            fn to_device() -> crate::backend::wgpu::WgpuDevice {
                crate::backend::wgpu::WgpuDevice::$device_variant $(($n))?
            }
        }
    };
}

impl_device!(WgpuBestAvailableDevice, BestAvailable);
impl_device!(WgpuIntegratedGpuDevice, IntegratedGpu, N);
impl_device!(WgpuVirtualGpuDevice, VirtualGpu, N);
impl_device!(WgpuCpuDevice, Cpu);
