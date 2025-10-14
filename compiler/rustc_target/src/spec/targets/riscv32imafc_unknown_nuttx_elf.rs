use crate::spec::{
    Arch, Cc, LinkerFlavor, Lld, Os, PanicStrategy, RelocModel, StandardLibrarySupport, Target,
    TargetMetadata, TargetOptions, TargetStandardLibrarySupport, cvs,
};

pub(crate) fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128".into(),
        llvm_target: "riscv32".into(),
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
        arch: Arch::RiscV32,

        options: TargetOptions {
            families: cvs!["unix"],
            os: Os::NuttX,
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            cpu: "generic-rv32".into(),
            max_atomic_width: Some(32),
            llvm_abiname: "ilp32f".into(),
            features: "+m,+a,+c,+f".into(),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            ..Default::default()
        },
    }
}
