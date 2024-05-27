use crate::spec::{Cc, Lld, LinkerFlavor, PanicStrategy, RelroLevel, Target, TargetOptions, TargetMetadata};

pub fn target() -> Target {
    Target {
        llvm_target: "aarch64-unknown-linux-gnu".into(),
        metadata: TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
        // target_endian: "little".to_string(),
        // target_c_int_width: "32".to_string(),
        pointer_width: 64,
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".into(),
        arch: "aarch64".into(),
        options: TargetOptions {
            executables: true,
            os: "optee".into(),
            vendor: "unknown".into(),
            linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::Yes),
            // abi_blacklist: super::arm_base::abi_blacklist(),
            dynamic_linking: false,
            has_rpath: true,
            max_atomic_width: Some(128),
            panic_strategy: PanicStrategy::Abort,
            position_independent_executables: true,
            relro_level: RelroLevel::Full,
            .. Default::default()
        }
    }
}