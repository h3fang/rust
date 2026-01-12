//@ compile-flags: -C opt-level=3 -C target-feature=+sve

//@ only-aarch64
#![crate_type = "lib"]
#![allow(incomplete_features, internal_features)]
#![feature(rustc_attrs, link_llvm_intrinsics, core_intrinsics)]

use std::intrinsics::sve::sve_select;

#[derive(Copy, Clone)]
#[rustc_scalable_vector(16)]
#[allow(non_camel_case_types)]
pub struct svbool_t(bool);

#[derive(Copy, Clone)]
#[rustc_scalable_vector(16)]
#[allow(non_camel_case_types)]
pub struct svint8_t(i8);

#[derive(Copy, Clone)]
#[rustc_scalable_vector(4)]
#[allow(non_camel_case_types)]
pub struct svint32_t(i32);

#[derive(Copy, Clone)]
#[rustc_scalable_vector(2)]
#[allow(non_camel_case_types)]
pub struct svuint64_t(u64);

#[derive(Copy, Clone)]
#[rustc_scalable_vector(2)]
#[allow(non_camel_case_types)]
pub struct svfloat64_t(f64);

#[no_mangle]
#[target_feature(enable = "sve")]
//CHECK-LABEL: select_int8(
pub unsafe fn select_int8(m: svbool_t, a: svint8_t, b: svint8_t) -> svint8_t {
    // CHECK: [[TMP0:%.*]] = select <vscale x 16 x i1> [[PG:%.*]], <vscale x 16 x i8> [[OP1:%.*]], <vscale x 16 x i8> [[OP2:%.*]]
    // CHECK-NEXT: ret <vscale x 16 x i8> [[TMP0:%.*]]
    sve_select(m, a, b)
}

#[no_mangle]
#[target_feature(enable = "sve")]
//CHECK-LABEL: select_int32(
pub unsafe fn select_int32(m: svbool_t, a: svint32_t, b: svint32_t) -> svint32_t {
    // CHECK: [[TMP0:%.*]] = tail call <vscale x 4 x i1> @llvm.aarch64.sve.convert.from.svbool.nxv4i1(<vscale x 16 x i1> [[PG:%.*]])
    // CHECK-NEXT: [[TMP1:%.*]] = select <vscale x 4 x i1> [[PG:%.*]], <vscale x 4 x i32> [[OP1:%.*]], <vscale x 4 x i32> [[OP1:%.*]]
    // CHECK-NEXT: ret <vscale x 4 x i32> [[TMP1:%.*]]
    sve_select(m, a, b)
}

#[no_mangle]
#[target_feature(enable = "sve")]
//CHECK-LABEL: select_uint64(
pub unsafe fn select_uint64(m: svbool_t, a: svuint64_t, b: svuint64_t) -> svuint64_t {
    // CHECK: [[TMP0:%.*]] = tail call <vscale x 2 x i1> @llvm.aarch64.sve.convert.from.svbool.nxv2i1(<vscale x 16 x i1> [[PG:%.*]])
    // CHECK-NEXT: [[TMP1:%.*]] = select <vscale x 2 x i1> [[PG:%.*]], <vscale x 2 x i64> [[OP1:%.*]], <vscale x 2 x i64> [[OP1:%.*]]
    // CHECK-NEXT: ret <vscale x 2 x i64> [[TMP1:%.*]]
    sve_select(m, a, b)
}

#[no_mangle]
#[target_feature(enable = "sve")]
//CHECK-LABEL: select_float64(
pub unsafe fn select_float64(m: svbool_t, a: svfloat64_t, b: svfloat64_t) -> svfloat64_t {
    // CHECK: [[TMP0:%.*]] = tail call <vscale x 2 x i1> @llvm.aarch64.sve.convert.from.svbool.nxv2i1(<vscale x 16 x i1> [[PG:%.*]])
    // CHECK-NEXT: [[TMP1:%.*]] = select <vscale x 2 x i1> [[PG:%.*]], <vscale x 2 x double> [[OP1:%.*]], <vscale x 2 x double> [[OP1:%.*]]
    // CHECK-NEXT: ret <vscale x 2 x double> [[TMP1:%.*]]
    sve_select(m, a, b)
}
