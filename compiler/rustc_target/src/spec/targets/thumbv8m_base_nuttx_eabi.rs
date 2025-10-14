// Targets the Cortex-M23 processor (Baseline ARMv8-M)

use crate::spec::{
    Abi, Arch, FloatAbi, Os, StandardLibrarySupport, Target, TargetMetadata, TargetOptions,
    TargetStandardLibrarySupport, base, cvs,
};

pub(crate) fn target() -> Target {
    Target {
        llvm_target: "thumbv8m.base-none-eabi".into(),
        metadata: TargetMetadata {
            description: None,
            tier: Some(3),
            host_tools: None,
            standard_library_support: Some(TargetStandardLibrarySupport::new(
                StandardLibrarySupport::Std,
                StandardLibrarySupport::Std,
            )),
        },
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".into(),
        arch: Arch::Arm,

        options: TargetOptions {
            families: cvs!["unix"],
            os: Os::NuttX,
            abi: Abi::Eabi,
            llvm_floatabi: Some(FloatAbi::Soft),
            // ARMv8-M baseline doesn't support unaligned loads/stores so we disable them
            // with +strict-align.
            features: "+strict-align".into(),
            max_atomic_width: Some(32),
            ..base::arm_none::opts()
        },
    }
}
