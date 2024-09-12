use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KindleFusionDevice<Device> {
    _device: std::marker::PhantomData<Device>,
}

#[cfg(all(feature = "wgpu"))]
impl<Device, /* GraphicsApi, */ FloatElement, IntElement>
    crate::device::KindleDevice<
        burn_fusion::Fusion<crate::backend::Wgpu<FloatElement, IntElement>>,
        // crate::backend::Fusion<crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>>,
    > for KindleFusionDevice<Device>
where
    Device: crate::device::KindleDevice<crate::backend::Wgpu<FloatElement, IntElement>>,
    // GraphicsApi: crate::backend::wgpu::GraphicsApi,
    FloatElement: crate::backend::wgpu::FloatElement,
    IntElement: crate::backend::wgpu::IntElement,
{
    fn to_device() -> <burn_fusion::Fusion<
        crate::backend::Wgpu<FloatElement, IntElement>, // crate::backend::Fusion<crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>
    > as burn::tensor::backend::Backend>::Device {
        Device::to_device()
    }
}

#[cfg(all(feature = "wgpu", feature = "autodiff"))]
impl<Device, /* GraphicsApi, */ FloatElement, IntElement>
    crate::device::KindleDevice<
        crate::backend::Autodiff<
            burn_fusion::Fusion<crate::backend::Wgpu<FloatElement, IntElement>>,
            // crate::backend::Fusion<crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>>,
        >,
    > for KindleFusionDevice<Device>
where
    Device: crate::device::KindleDevice<crate::backend::Wgpu<FloatElement, IntElement>>,
    // GraphicsApi: crate::backend::wgpu::GraphicsApi,
    FloatElement: crate::backend::wgpu::FloatElement,
    IntElement: crate::backend::wgpu::IntElement,
{
    fn to_device() -> <burn_fusion::Fusion<
        crate::backend::Wgpu<FloatElement, IntElement>, // crate::backend::Fusion<crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>
    > as burn::tensor::backend::Backend>::Device {
        Device::to_device()
    }
}
