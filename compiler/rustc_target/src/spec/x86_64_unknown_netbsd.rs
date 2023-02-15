use crate::spec::{Cc, LinkerFlavor, Lld, SanitizerSet, StackProbeType, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = super::netbsd_base::opts();
    base.cpu = "x86-64".into();
    base.max_atomic_width = Some(64);
    base.add_pre_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m64"]);
    base.stack_probes = StackProbeType::X86;
    base.supported_sanitizers = SanitizerSet::ADDRESS
        | SanitizerSet::CFI
        | SanitizerSet::LEAK
        | SanitizerSet::MEMORY
        | SanitizerSet::THREAD;
    base.supports_xray = true;

    Target {
        llvm_target: "x86_64-unknown-netbsd".into(),
        pointer_width: 64,
        data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
            .into(),
        arch: "x86_64".into(),
        options: TargetOptions { mcount: "__mcount".into(), ..base },
    }
}
