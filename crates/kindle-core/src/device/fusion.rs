use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KindleFusionDevice<Device, Backend>
where
    Backend: burn_fusion::FusionBackend,
    Device: crate::device::KindleDevice<'static, Backend>,
{
    device: std::marker::PhantomData<Device>,
    _backend: std::marker::PhantomData<Backend>,
}

#[cfg(feature = "wgpu")]
impl<Device, GraphicsApi, FloatElement, IntElement>
    crate::device::KindleDevice<
        'static,
        crate::backend::Fusion<crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>>,
    > for KindleFusionDevice<Device, crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>>
where
    Device: crate::device::KindleDevice<
        'static,
        crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>,
    >,
    GraphicsApi: crate::backend::wgpu::GraphicsApi,
    FloatElement: crate::backend::wgpu::FloatElement,
    IntElement: crate::backend::wgpu::IntElement,
{
    fn to_device() -> <crate::backend::Fusion<crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>> as burn::tensor::backend::Backend>::Device{
        Device::to_device()
    }
}
