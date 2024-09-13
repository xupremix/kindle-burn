use burn::backend::wgpu::{JitBackend, WgpuRuntime};

#[cfg(feature = "autodiff")]
use burn::backend::Autodiff;

use burn_fusion::Fusion;
use serde::{Deserialize, Serialize};

use super::KindleDevice;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KindleFusionDevice<Device> {
    _device: std::marker::PhantomData<Device>,
}

#[cfg(all(feature = "wgpu", not(feature = "autodiff")))]
impl<Device, FloatElement, IntElement>
    KindleDevice<Fusion<JitBackend<WgpuRuntime, FloatElement, IntElement>>>
    for KindleFusionDevice<Device>
where
    Device: KindleDevice<Fusion<JitBackend<WgpuRuntime, FloatElement, IntElement>>>,
    FloatElement: crate::backend::wgpu::FloatElement,
    IntElement: crate::backend::wgpu::IntElement,
{
    fn to_device(
    ) -> <Fusion<JitBackend<WgpuRuntime, FloatElement, IntElement>> as burn::prelude::Backend>::Device
    {
        Device::to_device()
    }
}

#[cfg(all(feature = "wgpu", feature = "autodiff"))]
impl<Device, FloatElement, IntElement>
    KindleDevice<Autodiff<Fusion<JitBackend<WgpuRuntime, FloatElement, IntElement>>>>
    for KindleFusionDevice<Device>
where
    Device: KindleDevice<Autodiff<Fusion<JitBackend<WgpuRuntime, FloatElement, IntElement>>>>,
    FloatElement: crate::backend::wgpu::FloatElement,
    IntElement: crate::backend::wgpu::IntElement,
{
    fn to_device() -> <Autodiff<Fusion<JitBackend<WgpuRuntime, FloatElement, IntElement>>> as burn::prelude::Backend>::Device{
        Device::to_device()
    }
}
