use serde::{Deserialize, Serialize};

macro_rules! tch_device {
    ($device:ident, $device_variant:ident $(,$n:ident)?) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $device $(<const $n: usize>)?;
        impl<'dv, $(const $n: usize,)? Element>
            crate::device::KindleDevice<
                'dv,
                crate::backend::LibTorch<Element>,
            > for $device $(<$n>)?
        {
        }
        impl $(<const $n: usize>)? From<$device $(<$n>)?>
            for crate::backend::libtorch::LibTorchDevice {
            fn from(_: $device $(<$n>)?) -> Self {
                Self::$device_variant $(($n))?
            }
        }
        impl $(<const $n: usize>)? From<&$device $(<$n>)?>
            for crate::backend::libtorch::LibTorchDevice {
            fn from(_: &$device $(<$n>)?) -> Self {
                Self::$device_variant $(($n))?
            }
        }
    };
}

tch_device!(LibTorchCudaDevice, Cuda, N);
tch_device!(LibTorchCpuDevice, Cpu);
tch_device!(LibTorchMpsDevice, Mps);
tch_device!(LibTorchVulkanDevice, Vulkan);
