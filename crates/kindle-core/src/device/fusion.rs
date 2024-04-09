use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FusionDevice<'dv, Device, Backend, DeviceBackend>
where
    Backend: burn_fusion::FusionBackend,
    DeviceBackend: crate::tensor::backend::Backend,
    Device: crate::device::KindleDevice<'dv, DeviceBackend>,
{
    pub device: Device,
}

impl<'dv, Device, Backend> crate::device::KindleDevice<'dv, crate::backend::Fusion<Backend>>
    for FusionDevice<Device, crate::backend::Fusion<Backend>>
where
    Backend: crate::tensor::backend::Backend,
    Device: crate::device::KindleDevice<'dv, Backend>,
{
}

impl<'dv, Device, Backend> From<FusionDevice<Device, Backend>>
    for <Backend as burn_fusion::FusionBackend>::FusionDevice
where
    Backend: burn_fusion::FusionBackend,
    Device: crate::device::KindleDevice<'dv, Backend>,
{
    fn from(device: FusionDevice<Device, Backend>) -> Self {
        device.device.into()
    }
}
