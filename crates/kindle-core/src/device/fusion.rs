use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KindleFusionDevice<Device, Backend>
where
    Backend: burn_fusion::FusionBackend,
    Device: crate::device::KindleDevice<Backend>,
{
    _device: std::marker::PhantomData<Device>,
    _backend: std::marker::PhantomData<Backend>,
}

#[cfg(all(feature = "wgpu", not(feature = "autodiff")))]
impl<Device, GraphicsApi, FloatElement, IntElement>
    crate::device::KindleDevice<
        crate::backend::Fusion<crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>>,
    > for KindleFusionDevice<Device, crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>>
where
    Device:
        crate::device::KindleDevice<crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>>,
    GraphicsApi: crate::backend::wgpu::GraphicsApi,
    FloatElement: crate::backend::wgpu::FloatElement,
    IntElement: crate::backend::wgpu::IntElement,
{
    fn to_device() -> <crate::backend::Fusion<crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>> as burn::tensor::backend::Backend>::Device{
        Device::to_device()
    }
}

#[cfg(all(feature = "wgpu", feature = "autodiff"))]
impl<Device, GraphicsApi, FloatElement, IntElement>
    crate::device::KindleDevice<
        crate::backend::Autodiff<
            crate::backend::Fusion<crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>>,
        >,
    > for KindleFusionDevice<Device, crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>>
where
    Device:
        crate::device::KindleDevice<crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>>,
    GraphicsApi: crate::backend::wgpu::GraphicsApi,
    FloatElement: crate::backend::wgpu::FloatElement,
    IntElement: crate::backend::wgpu::IntElement,
{
    fn to_device() -> <crate::backend::Fusion<crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>> as burn::tensor::backend::Backend>::Device{
        Device::to_device()
    }
}
