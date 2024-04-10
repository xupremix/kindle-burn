use serde::{Deserialize, Serialize};

macro_rules! wgpu_device {
    ($device:ident, $device_variant:ident $(,$n:ident)?) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $device <
            $(const $n: usize = 0,)?
            GraphicsApi = crate::backend::wgpu::AutoGraphicsApi,
            FloatElement = f32,
            IntElement = i32,
        >
        where
            GraphicsApi: crate::backend::wgpu::GraphicsApi,
            FloatElement: crate::backend::wgpu::FloatElement,
            IntElement: crate::backend::wgpu::IntElement,
        {
            _graphics_api: std::marker::PhantomData<GraphicsApi>,
            _float_element: std::marker::PhantomData<FloatElement>,
            _int_element: std::marker::PhantomData<IntElement>,
        }
        impl<
            $(const $n: usize,)?
            GraphicsApi,
            FloatElement,
            IntElement,
        >   crate::device::KindleDevice<
            crate::backend::Wgpu<GraphicsApi, FloatElement, IntElement>,
        > for $device <$($n,)? GraphicsApi, FloatElement, IntElement>
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

wgpu_device!(WgpuBestAvailableDevice, BestAvailable);
wgpu_device!(WgpuIntegratedGpuDevice, IntegratedGpu, N);
wgpu_device!(WgpuVirtualGpuDevice, VirtualGpu, N);
wgpu_device!(WgpuCpuDevice, Cpu);
