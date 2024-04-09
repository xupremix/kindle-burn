use serde::{Deserialize, Serialize};

macro_rules! wgpu_device {
    ($device:ident, $device_variant:ident $(,$n:ident)?) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $device $(<const $n: usize>)?;
        impl<'dv, $(const $n: usize,)? GraphicsApi, FloatElement, IntElement>
            crate::device::KindleDevice<
                'dv,
                crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>,
            > for $device $(<$n>)?
        where
            GraphicsApi: crate::backend::wgpu::GraphicsApi,
            FloatElement: crate::backend::wgpu::FloatElement,
            IntElement: crate::backend::wgpu::IntElement,
        {
        }
        impl $(<const $n: usize>)? From<$device $(<$n>)?>
            for crate::backend::wgpu::WgpuDevice {
            fn from(_: $device $(<$n>)?) -> Self {
                Self::$device_variant $(($n))?
            }
        }
        impl $(<const $n: usize>)? From<&$device $(<$n>)?>
            for crate::backend::wgpu::WgpuDevice {
            fn from(_: &$device $(<$n>)?) -> Self {
                Self::$device_variant $(($n))?
            }
        }
    };
}

wgpu_device!(WgpuIntegratedGpuDevice, IntegratedGpu, N);
wgpu_device!(WgpuVirtualGpuDevice, VirtualGpu, N);
wgpu_device!(WgpuCpuDevice, Cpu);
wgpu_device!(WgpuBestAvailableDevice, BestAvailable);
