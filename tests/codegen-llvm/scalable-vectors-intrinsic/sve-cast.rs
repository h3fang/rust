//@ compile-flags: -C opt-level=3 -C target-feature=+sve

//@ only-aarch64
#![crate_type = "lib"]
#![allow(incomplete_features, internal_features)]
#![feature(rustc_attrs, link_llvm_intrinsics, core_intrinsics)]

use std::intrinsics::sve::sve_cast;

#[derive(Copy, Clone)]
#[rustc_scalable_vector(2)]
#[allow(non_camel_case_types)]
pub struct nxv2i32(i32);

#[derive(Copy, Clone)]
#[rustc_scalable_vector(2)]
#[allow(non_camel_case_types)]
pub struct nxv2i64(i64);

#[derive(Copy, Clone)]
#[rustc_scalable_vector(2)]
#[allow(non_camel_case_types)]
pub struct nxv2f32(f32);

#[no_mangle]
#[target_feature(enable = "sve")]
// CHECK-LABEL: cast_i32_f32
pub unsafe fn cast_i32_f32(a: nxv2i32) -> nxv2f32 {
    // CHECK: %0 = sitofp <vscale x 2 x i32> %a to <vscale x 2 x float>
    // CHECK-NEXT: ret <vscale x 2 x float> %0
    sve_cast(a)
}

#[no_mangle]
#[target_feature(enable = "sve")]
// CHECK-LABEL: cast_f32_i32
pub unsafe fn cast_f32_i32(a: nxv2f32) -> nxv2i32 {
    // CHECK: %0 = fptosi <vscale x 2 x float> %a to <vscale x 2 x i32>
    // CHECK-NEXT: ret <vscale x 2 x i32> %0
    sve_cast(a)
}

#[no_mangle]
#[target_feature(enable = "sve")]
// CHECK-LABEL: cast_i32_i64
pub unsafe fn cast_i32_i64(a: nxv2i32) -> nxv2i64 {
    // CHECK: %0 = sext <vscale x 2 x i32> %a to <vscale x 2 x i64>
    // CHECK-NEXT: ret <vscale x 2 x i64> %0
    sve_cast(a)
}

#[no_mangle]
#[target_feature(enable = "sve")]
// CHECK-LABEL: cast_i64_i32
pub unsafe fn cast_i64_i32(a: nxv2i64) -> nxv2i32 {
    // CHECK: %0 = trunc <vscale x 2 x i64> %a to <vscale x 2 x i32>
    // CHECK-NEXT: ret <vscale x 2 x i32> %0
    sve_cast(a)
}

#[no_mangle]
#[target_feature(enable = "sve")]
pub unsafe fn cast_i32_i32(a: nxv2i32) -> nxv2i32 {
    // CHECK: ret <vscale x 2 x i32> %a
    sve_cast(a)
}
