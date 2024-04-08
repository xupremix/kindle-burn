pub struct WgpuDiscreteGpuDevice<const T: usize>;
impl<const T: usize> crate::Sealed for WgpuDiscreteGpuDevice<T> {}
impl<const T: usize> crate::device::KindleDevice for WgpuDiscreteGpuDevice<T> {}

pub struct WgpuIntegratedGpuDevice<const T: usize>;
impl<const T: usize> crate::Sealed for WgpuIntegratedGpuDevice<T> {}
impl<const T: usize> crate::device::KindleDevice for WgpuIntegratedGpuDevice<T> {}

pub struct WgpuVirtualGpuDevice<const T: usize>;
impl<const T: usize> crate::Sealed for WgpuVirtualGpuDevice<T> {}
impl<const T: usize> crate::device::KindleDevice for WgpuVirtualGpuDevice<T> {}

pub struct WgpuCpuDevice;
impl crate::Sealed for WgpuCpuDevice {}
impl crate::device::KindleDevice for WgpuCpuDevice {}

pub struct WgpuBestAvailableDevice;
impl crate::Sealed for WgpuBestAvailableDevice {}
impl crate::device::KindleDevice for WgpuBestAvailableDevice {}

impl<const T: usize> From<WgpuDiscreteGpuDevice<T>> for crate::backend::wgpu::WgpuDevice {
    fn from(_: WgpuDiscreteGpuDevice<T>) -> Self {
        Self::DiscreteGpu(T)
    }
}

impl<const T: usize> From<WgpuIntegratedGpuDevice<T>> for crate::backend::wgpu::WgpuDevice {
    fn from(_: WgpuIntegratedGpuDevice<T>) -> Self {
        Self::IntegratedGpu(T)
    }
}

impl<const T: usize> From<WgpuVirtualGpuDevice<T>> for crate::backend::wgpu::WgpuDevice {
    fn from(_: WgpuVirtualGpuDevice<T>) -> Self {
        Self::VirtualGpu(T)
    }
}

impl From<WgpuCpuDevice> for crate::backend::wgpu::WgpuDevice {
    fn from(_: WgpuCpuDevice) -> Self {
        Self::Cpu
    }
}

impl From<WgpuBestAvailableDevice> for crate::backend::wgpu::WgpuDevice {
    fn from(_: WgpuBestAvailableDevice) -> Self {
        Self::BestAvailable
    }
}
