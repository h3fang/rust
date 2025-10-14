use crate::spec::{
    Arch, Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, StandardLibrarySupport, Target,
    TargetMetadata, TargetOptions, TargetStandardLibrarySupport,
};

pub(crate) fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128".into(),
        llvm_target: "riscv32".into(),
        metadata: TargetMetadata {
            description: Some("Bare RISC-V (RV32IMC ISA)".into()),
            tier: Some(2),
            host_tools: Some(false),
            standard_library_support: Some(TargetStandardLibrarySupport::new(
                StandardLibrarySupport::Core,
                StandardLibrarySupport::Core,
            )),
        },
        pointer_width: 32,
        arch: Arch::RiscV32,

        options: TargetOptions {
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            cpu: "generic-rv32".into(),
            max_atomic_width: Some(32),
            atomic_cas: false,
            features: "+m,+c,+forced-atomics".into(),
            llvm_abiname: "ilp32".into(),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            ..Default::default()
        },
    }
}
