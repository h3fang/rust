use rustc_abi::Endian;

use crate::spec::{
    Arch, StandardLibrarySupport, Target, TargetMetadata, TargetStandardLibrarySupport, base,
};

pub(crate) fn target() -> Target {
    Target {
        llvm_target: "bpfeb".into(),
        metadata: TargetMetadata {
            description: Some("BPF (big endian)".into()),
            tier: Some(3),
            host_tools: Some(false),
            standard_library_support: Some(TargetStandardLibrarySupport::new(
                StandardLibrarySupport::Core,
                StandardLibrarySupport::Core,
            )),
        },
        data_layout: "E-m:e-p:64:64-i64:64-i128:128-n32:64-S128".into(),
        pointer_width: 64,
        arch: Arch::Bpf,
        options: base::bpf::opts(Endian::Big),
    }
}
