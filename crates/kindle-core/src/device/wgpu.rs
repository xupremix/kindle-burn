use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WgpuDiscreteGpuDevice<const N: usize>;
impl<const N: usize> crate::Sealed for WgpuDiscreteGpuDevice<N> {}
impl<const N: usize> crate::device::KindleDevice<'_> for WgpuDiscreteGpuDevice<N> {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WgpuIntegratedGpuDevice<const N: usize>;
impl<const N: usize> crate::Sealed for WgpuIntegratedGpuDevice<N> {}
impl<const N: usize> crate::device::KindleDevice<'_> for WgpuIntegratedGpuDevice<N> {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WgpuVirtualGpuDevice<const N: usize>;
impl<const N: usize> crate::Sealed for WgpuVirtualGpuDevice<N> {}
impl<const N: usize> crate::device::KindleDevice<'_> for WgpuVirtualGpuDevice<N> {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WgpuCpuDevice;
impl crate::Sealed for WgpuCpuDevice {}
impl crate::device::KindleDevice<'_> for WgpuCpuDevice {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WgpuBestAvailableDevice;
impl crate::Sealed for WgpuBestAvailableDevice {}
impl crate::device::KindleDevice<'_> for WgpuBestAvailableDevice {}

// Taking ownership
impl<const N: usize> From<WgpuDiscreteGpuDevice<N>> for crate::backend::wgpu::WgpuDevice {
    fn from(_: WgpuDiscreteGpuDevice<N>) -> Self {
        Self::DiscreteGpu(N)
    }
}

impl<const N: usize> From<WgpuIntegratedGpuDevice<N>> for crate::backend::wgpu::WgpuDevice {
    fn from(_: WgpuIntegratedGpuDevice<N>) -> Self {
        Self::IntegratedGpu(N)
    }
}

impl<const N: usize> From<WgpuVirtualGpuDevice<N>> for crate::backend::wgpu::WgpuDevice {
    fn from(_: WgpuVirtualGpuDevice<N>) -> Self {
        Self::VirtualGpu(N)
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

// Not taking ownershp
impl<const N: usize> From<&WgpuDiscreteGpuDevice<N>> for crate::backend::wgpu::WgpuDevice {
    fn from(_: &WgpuDiscreteGpuDevice<N>) -> Self {
        Self::DiscreteGpu(N)
    }
}

impl<const N: usize> From<&WgpuIntegratedGpuDevice<N>> for crate::backend::wgpu::WgpuDevice {
    fn from(_: &WgpuIntegratedGpuDevice<N>) -> Self {
        Self::IntegratedGpu(N)
    }
}

impl<const N: usize> From<&WgpuVirtualGpuDevice<N>> for crate::backend::wgpu::WgpuDevice {
    fn from(_: &WgpuVirtualGpuDevice<N>) -> Self {
        Self::VirtualGpu(N)
    }
}

impl From<&WgpuCpuDevice> for crate::backend::wgpu::WgpuDevice {
    fn from(_: &WgpuCpuDevice) -> Self {
        Self::Cpu
    }
}

impl From<&WgpuBestAvailableDevice> for crate::backend::wgpu::WgpuDevice {
    fn from(_: &WgpuBestAvailableDevice) -> Self {
        Self::BestAvailable
    }
}
