; ModuleID = 'f.a4af07ec-cgu.0'
source_filename = "f.a4af07ec-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"std::io::error::ErrorData<alloc::boxed::Box<std::io::error::Custom>>" = type { i8, [15 x i8] }
%"std::io::error::ErrorData<alloc::boxed::Box<std::io::error::Custom>>::Os" = type { [1 x i32], i32 }
%"std::io::error::ErrorData<alloc::boxed::Box<std::io::error::Custom>>::SimpleMessage" = type { [1 x i64], ptr }
%"std::io::error::ErrorData<alloc::boxed::Box<std::io::error::Custom>>::Custom" = type { [1 x i64], ptr }
%"std::io::error::ErrorData<alloc::boxed::Box<std::io::error::Custom>>::Simple" = type { [1 x i8], i8 }
%"core::fmt::Arguments<'_>" = type { { ptr, i64 }, { ptr, i64 }, { ptr, i64 } }
%"core::ptr::metadata::PtrComponents<()>" = type { ptr, {} }
%"core::ptr::metadata::PtrRepr<()>" = type { [1 x i64] }
%"core::ptr::metadata::PtrComponents<u8>" = type { ptr, {} }
%"core::ptr::metadata::PtrRepr<u8>" = type { [1 x i64] }
%"core::result::Result<usize, std::io::error::Error>::Ok" = type { [1 x i64], i64 }
%"core::result::Result<usize, std::io::error::Error>::Err" = type { [1 x i64], ptr }
%"alloc::alloc::Global" = type {}
%"alloc::string::String" = type { %"alloc::vec::Vec<u8>" }
%"alloc::vec::Vec<u8>" = type { { i64, ptr }, i64 }
%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>" = type { [2 x i64], i64 }
%"core::ptr::metadata::PtrRepr<[u8]>" = type { [2 x i64] }
%"e<alloc::string::String, i32>" = type { [1 x i64], ptr, [1 x i64] }
%"core::result::Result<usize, std::io::error::Error>" = type { i64, [1 x i64] }

@alloc120 = private unnamed_addr constant <{ [40 x i8] }> <{ [40 x i8] c"internal error: entered unreachable code" }>, align 1
@alloc121 = private unnamed_addr constant <{ [90 x i8] }> <{ [90 x i8] c"/rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/io/error/repr_bitpacked.rs" }>, align 1
@alloc122 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc121, [16 x i8] c"Z\00\00\00\00\00\00\00\18\01\00\00\0D\00\00\00" }>, align 8
@vtable.0 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr }> <{ ptr @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17he8fd658a00d261a8E", [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h96d407bc8986291dE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf7706a4bc99c980bE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf7706a4bc99c980bE" }>, align 8
@alloc24 = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"invalid args" }>, align 1
@alloc25 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc24, [8 x i8] c"\0C\00\00\00\00\00\00\00" }>, align 8
@alloc13 = private unnamed_addr constant <{}> zeroinitializer, align 8
@alloc128 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/fmt/mod.rs" }>, align 1
@alloc129 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc128, [16 x i8] c"K\00\00\00\00\00\00\00\8C\01\00\00\0D\00\00\00" }>, align 8
@alloc130 = private unnamed_addr constant <{ [80 x i8] }> <{ [80 x i8] c"/rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/alloc/layout.rs" }>, align 1
@alloc131 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc130, [16 x i8] c"P\00\00\00\00\00\00\00\C4\01\00\00)\00\00\00" }>, align 8
@str.1 = internal constant [25 x i8] c"attempt to divide by zero"
@alloc132 = private unnamed_addr constant <{ [43 x i8] }> <{ [43 x i8] c"called `Result::unwrap()` on an `Err` value" }>, align 1
@vtable.2 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @"_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h16f704f8a58dbcb2E", [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN58_$LT$std..io..error..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h5f20ceb8e82723c0E" }>, align 8
@0 = private unnamed_addr constant <{ [16 x i8] }> <{ [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00" }>, align 8
@alloc136 = private unnamed_addr constant <{ [76 x i8] }> <{ [76 x i8] c"/rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/raw_vec.rs" }>, align 1
@alloc137 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc136, [16 x i8] c"L\00\00\00\00\00\00\00\F7\00\00\00;\00\00\00" }>, align 8
@alloc3 = private unnamed_addr constant <{ [18 x i8] }> <{ [18 x i8] c"Enter your name :\0A" }>, align 1
@alloc4 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc3, [8 x i8] c"\12\00\00\00\00\00\00\00" }>, align 8
@alloc138 = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"f.rs" }>, align 1
@alloc139 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc138, [16 x i8] c"\04\00\00\00\00\00\00\00\0A\00\00\004\00\00\00" }>, align 8
@alloc15 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"\0A" }>, align 1
@alloc14 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc13, [8 x i8] zeroinitializer, ptr @alloc15, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8

; <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
; Function Attrs: inlinehint nonlazybind uwtable
define internal ptr @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h69c375d0365376d4E"(ptr %unique) unnamed_addr #0 {
start:
  %0 = alloca ptr, align 8
  store ptr %unique, ptr %0, align 8
  %1 = load ptr, ptr %0, align 8, !nonnull !3, !noundef !3
  ret ptr %1
}

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nonlazybind uwtable
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h5e897f3271c6a419E(ptr %f) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca { ptr, i32 }, align 8
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17ha8340e8701cd160aE(ptr %f)
  call void asm sideeffect "", "~{memory}"(), !srcloc !4
  br label %bb4

bb4:                                              ; preds = %start
  ret void

bb2:                                              ; No predecessors!
  %1 = load ptr, ptr %0, align 8
  %2 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  %3 = load i32, ptr %2, align 8
  %4 = insertvalue { ptr, i32 } undef, ptr %1, 0
  %5 = insertvalue { ptr, i32 } %4, i32 %3, 1
  resume { ptr, i32 } %5
}

; std::io::error::repr_bitpacked::decode_repr
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN3std2io5error14repr_bitpacked11decode_repr17h1e517f8878b0c3b5E(ptr sret(%"std::io::error::ErrorData<alloc::boxed::Box<std::io::error::Custom>>") %0, ptr %ptr) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %1 = alloca ptr, align 8
  %2 = alloca { ptr, i32 }, align 8
  %_36 = alloca i8, align 1
  %_33 = alloca ptr, align 8
  %self1 = alloca ptr, align 8
  %f = alloca ptr, align 8
  %self = alloca i8, align 1
  %kind = alloca i8, align 1
  %bits = alloca i64, align 8
  store i8 1, ptr %_36, align 1
  store ptr %ptr, ptr %bits, align 8
  br label %bb14

bb14:                                             ; preds = %start
  %_7 = load i64, ptr %bits, align 8
  %_6 = and i64 %_7, 3
  switch i64 %_6, label %bb1 [
    i64 2, label %bb2
    i64 3, label %bb3
    i64 0, label %bb5
    i64 1, label %bb6
  ]

bb1:                                              ; preds = %bb14
; invoke core::panicking::panic
  invoke void @_ZN4core9panicking5panic17h341545107301821dE(ptr align 1 @alloc120, i64 40, ptr align 8 @alloc122) #11
          to label %unreachable unwind label %cleanup

bb2:                                              ; preds = %bb14
  %_11 = load i64, ptr %bits, align 8
  %_9 = ashr i64 %_11, 32
  %code = trunc i64 %_9 to i32
  %3 = getelementptr inbounds %"std::io::error::ErrorData<alloc::boxed::Box<std::io::error::Custom>>::Os", ptr %0, i32 0, i32 1
  store i32 %code, ptr %3, align 4
  store i8 0, ptr %0, align 8
  br label %bb8

bb3:                                              ; preds = %bb14
  %_15 = load i64, ptr %bits, align 8
  %_14 = lshr i64 %_15, 32
  %kind_bits = trunc i64 %_14 to i32
; invoke std::io::error::repr_bitpacked::kind_from_prim
  %4 = invoke i8 @_ZN3std2io5error14repr_bitpacked14kind_from_prim17h5e02de8ccc5fd30fE(i32 %kind_bits)
          to label %bb4 unwind label %cleanup, !range !5

bb5:                                              ; preds = %bb14
  store ptr %ptr, ptr %self1, align 8
  %_51 = load ptr, ptr %self1, align 8
  %5 = getelementptr inbounds %"std::io::error::ErrorData<alloc::boxed::Box<std::io::error::Custom>>::SimpleMessage", ptr %0, i32 0, i32 1
  store ptr %_51, ptr %5, align 8
  store i8 2, ptr %0, align 8
  br label %bb8

bb6:                                              ; preds = %bb14
  %6 = getelementptr i8, ptr %ptr, i64 -1
  store ptr %6, ptr %1, align 8
  %_66 = load ptr, ptr %1, align 8
  br label %bb21

bb13:                                             ; preds = %cleanup
  %7 = load i8, ptr %_36, align 1, !range !6, !noundef !3
  %8 = trunc i8 %7 to i1
  br i1 %8, label %bb12, label %bb10

cleanup:                                          ; preds = %bb15, %bb3, %bb20, %bb21, %bb1
  %9 = landingpad { ptr, i32 }
          cleanup
  %10 = extractvalue { ptr, i32 } %9, 0
  %11 = extractvalue { ptr, i32 } %9, 1
  %12 = getelementptr inbounds { ptr, i32 }, ptr %2, i32 0, i32 0
  store ptr %10, ptr %12, align 8
  %13 = getelementptr inbounds { ptr, i32 }, ptr %2, i32 0, i32 1
  store i32 %11, ptr %13, align 8
  br label %bb13

unreachable:                                      ; preds = %bb1
  unreachable

bb21:                                             ; preds = %bb6
; invoke core::ptr::mut_ptr::<impl *mut T>::with_metadata_of
  %self2 = invoke ptr @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$16with_metadata_of17hecc9aa2ec60c4821E"(ptr %_66, ptr %ptr)
          to label %bb20 unwind label %cleanup

bb20:                                             ; preds = %bb21
  store i8 0, ptr %_36, align 1
  store ptr %self2, ptr %_33, align 8
  %14 = load ptr, ptr %_33, align 8
; invoke <std::io::error::repr_bitpacked::Repr as core::ops::drop::Drop>::drop::{{closure}}
  %_31 = invoke align 8 ptr @"_ZN78_$LT$std..io..error..repr_bitpacked..Repr$u20$as$u20$core..ops..drop..Drop$GT$4drop28_$u7b$$u7b$closure$u7d$$u7d$17h7f1769e47fcd9dc1E"(ptr %14)
          to label %bb7 unwind label %cleanup

bb7:                                              ; preds = %bb20
  %15 = getelementptr inbounds %"std::io::error::ErrorData<alloc::boxed::Box<std::io::error::Custom>>::Custom", ptr %0, i32 0, i32 1
  store ptr %_31, ptr %15, align 8
  store i8 3, ptr %0, align 8
  br label %bb8

bb8:                                              ; preds = %bb2, %bb19, %bb5, %bb7
  %16 = load i8, ptr %_36, align 1, !range !6, !noundef !3
  %17 = trunc i8 %16 to i1
  br i1 %17, label %bb11, label %bb9

bb4:                                              ; preds = %bb3
  store i8 %4, ptr %self, align 1
  store ptr %bits, ptr %f, align 8
  %18 = load i8, ptr %self, align 1, !range !5, !noundef !3
  %19 = icmp eq i8 %18, 41
  %_39 = select i1 %19, i64 0, i64 1
  %20 = icmp eq i64 %_39, 0
  br i1 %20, label %bb15, label %bb17

bb15:                                             ; preds = %bb4
  %_41 = load ptr, ptr %f, align 8, !nonnull !3, !align !7, !noundef !3
; invoke std::io::error::repr_bitpacked::decode_repr::{{closure}}
  %21 = invoke i8 @"_ZN3std2io5error14repr_bitpacked11decode_repr28_$u7b$$u7b$closure$u7d$$u7d$17h78efe98282fa589fE"(ptr align 8 %_41)
          to label %bb18 unwind label %cleanup, !range !8

bb17:                                             ; preds = %bb4
  %x = load i8, ptr %self, align 1, !range !8, !noundef !3
  store i8 %x, ptr %kind, align 1
  br label %bb19

bb16:                                             ; No predecessors!
  unreachable

bb19:                                             ; preds = %bb18, %bb17
  %_21 = load i8, ptr %kind, align 1, !range !8, !noundef !3
  %22 = getelementptr inbounds %"std::io::error::ErrorData<alloc::boxed::Box<std::io::error::Custom>>::Simple", ptr %0, i32 0, i32 1
  store i8 %_21, ptr %22, align 1
  store i8 1, ptr %0, align 8
  br label %bb8

bb18:                                             ; preds = %bb15
  store i8 %21, ptr %kind, align 1
  br label %bb19

bb10:                                             ; preds = %bb12, %bb13
  %23 = load ptr, ptr %2, align 8
  %24 = getelementptr inbounds { ptr, i32 }, ptr %2, i32 0, i32 1
  %25 = load i32, ptr %24, align 8
  %26 = insertvalue { ptr, i32 } undef, ptr %23, 0
  %27 = insertvalue { ptr, i32 } %26, i32 %25, 1
  resume { ptr, i32 } %27

bb12:                                             ; preds = %bb13
  br label %bb10

bb9:                                              ; preds = %bb11, %bb8
  ret void

bb11:                                             ; preds = %bb8
  br label %bb9
}

; std::io::error::repr_bitpacked::decode_repr::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN3std2io5error14repr_bitpacked11decode_repr28_$u7b$$u7b$closure$u7d$$u7d$17h78efe98282fa589fE"(ptr align 8 %_1) unnamed_addr #0 {
start:
; call core::hint::unreachable_unchecked
  call void @_ZN4core4hint21unreachable_unchecked17h19f9a20c320cdb73E() #11
  unreachable
}

; std::io::error::repr_bitpacked::kind_from_prim
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @_ZN3std2io5error14repr_bitpacked14kind_from_prim17h5e02de8ccc5fd30fE(i32 %0) unnamed_addr #0 {
start:
  %1 = alloca i8, align 1
  %ek = alloca i32, align 4
  store i32 %0, ptr %ek, align 4
  %_5 = load i32, ptr %ek, align 4
  %_4 = icmp eq i32 %_5, 0
  br i1 %_4, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %_10 = load i32, ptr %ek, align 4
  %_9 = icmp eq i32 %_10, 1
  br i1 %_9, label %bb3, label %bb4

bb1:                                              ; preds = %start
  store i8 0, ptr %1, align 1
  br label %bb83

bb83:                                             ; preds = %bb82, %bb81, %bb79, %bb77, %bb75, %bb73, %bb71, %bb69, %bb67, %bb65, %bb63, %bb61, %bb59, %bb57, %bb55, %bb53, %bb51, %bb49, %bb47, %bb45, %bb43, %bb41, %bb39, %bb37, %bb35, %bb33, %bb31, %bb29, %bb27, %bb25, %bb23, %bb21, %bb19, %bb17, %bb15, %bb13, %bb11, %bb9, %bb7, %bb5, %bb3, %bb1
  %2 = load i8, ptr %1, align 1, !range !5, !noundef !3
  ret i8 %2

bb4:                                              ; preds = %bb2
  %_15 = load i32, ptr %ek, align 4
  %_14 = icmp eq i32 %_15, 2
  br i1 %_14, label %bb5, label %bb6

bb3:                                              ; preds = %bb2
  store i8 1, ptr %1, align 1
  br label %bb83

bb6:                                              ; preds = %bb4
  %_20 = load i32, ptr %ek, align 4
  %_19 = icmp eq i32 %_20, 3
  br i1 %_19, label %bb7, label %bb8

bb5:                                              ; preds = %bb4
  store i8 2, ptr %1, align 1
  br label %bb83

bb8:                                              ; preds = %bb6
  %_25 = load i32, ptr %ek, align 4
  %_24 = icmp eq i32 %_25, 4
  br i1 %_24, label %bb9, label %bb10

bb7:                                              ; preds = %bb6
  store i8 3, ptr %1, align 1
  br label %bb83

bb10:                                             ; preds = %bb8
  %_30 = load i32, ptr %ek, align 4
  %_29 = icmp eq i32 %_30, 5
  br i1 %_29, label %bb11, label %bb12

bb9:                                              ; preds = %bb8
  store i8 4, ptr %1, align 1
  br label %bb83

bb12:                                             ; preds = %bb10
  %_35 = load i32, ptr %ek, align 4
  %_34 = icmp eq i32 %_35, 6
  br i1 %_34, label %bb13, label %bb14

bb11:                                             ; preds = %bb10
  store i8 5, ptr %1, align 1
  br label %bb83

bb14:                                             ; preds = %bb12
  %_40 = load i32, ptr %ek, align 4
  %_39 = icmp eq i32 %_40, 7
  br i1 %_39, label %bb15, label %bb16

bb13:                                             ; preds = %bb12
  store i8 6, ptr %1, align 1
  br label %bb83

bb16:                                             ; preds = %bb14
  %_45 = load i32, ptr %ek, align 4
  %_44 = icmp eq i32 %_45, 8
  br i1 %_44, label %bb17, label %bb18

bb15:                                             ; preds = %bb14
  store i8 7, ptr %1, align 1
  br label %bb83

bb18:                                             ; preds = %bb16
  %_50 = load i32, ptr %ek, align 4
  %_49 = icmp eq i32 %_50, 9
  br i1 %_49, label %bb19, label %bb20

bb17:                                             ; preds = %bb16
  store i8 8, ptr %1, align 1
  br label %bb83

bb20:                                             ; preds = %bb18
  %_55 = load i32, ptr %ek, align 4
  %_54 = icmp eq i32 %_55, 10
  br i1 %_54, label %bb21, label %bb22

bb19:                                             ; preds = %bb18
  store i8 9, ptr %1, align 1
  br label %bb83

bb22:                                             ; preds = %bb20
  %_60 = load i32, ptr %ek, align 4
  %_59 = icmp eq i32 %_60, 11
  br i1 %_59, label %bb23, label %bb24

bb21:                                             ; preds = %bb20
  store i8 10, ptr %1, align 1
  br label %bb83

bb24:                                             ; preds = %bb22
  %_65 = load i32, ptr %ek, align 4
  %_64 = icmp eq i32 %_65, 12
  br i1 %_64, label %bb25, label %bb26

bb23:                                             ; preds = %bb22
  store i8 11, ptr %1, align 1
  br label %bb83

bb26:                                             ; preds = %bb24
  %_70 = load i32, ptr %ek, align 4
  %_69 = icmp eq i32 %_70, 13
  br i1 %_69, label %bb27, label %bb28

bb25:                                             ; preds = %bb24
  store i8 12, ptr %1, align 1
  br label %bb83

bb28:                                             ; preds = %bb26
  %_75 = load i32, ptr %ek, align 4
  %_74 = icmp eq i32 %_75, 14
  br i1 %_74, label %bb29, label %bb30

bb27:                                             ; preds = %bb26
  store i8 13, ptr %1, align 1
  br label %bb83

bb30:                                             ; preds = %bb28
  %_80 = load i32, ptr %ek, align 4
  %_79 = icmp eq i32 %_80, 15
  br i1 %_79, label %bb31, label %bb32

bb29:                                             ; preds = %bb28
  store i8 14, ptr %1, align 1
  br label %bb83

bb32:                                             ; preds = %bb30
  %_85 = load i32, ptr %ek, align 4
  %_84 = icmp eq i32 %_85, 16
  br i1 %_84, label %bb33, label %bb34

bb31:                                             ; preds = %bb30
  store i8 15, ptr %1, align 1
  br label %bb83

bb34:                                             ; preds = %bb32
  %_90 = load i32, ptr %ek, align 4
  %_89 = icmp eq i32 %_90, 17
  br i1 %_89, label %bb35, label %bb36

bb33:                                             ; preds = %bb32
  store i8 16, ptr %1, align 1
  br label %bb83

bb36:                                             ; preds = %bb34
  %_95 = load i32, ptr %ek, align 4
  %_94 = icmp eq i32 %_95, 18
  br i1 %_94, label %bb37, label %bb38

bb35:                                             ; preds = %bb34
  store i8 17, ptr %1, align 1
  br label %bb83

bb38:                                             ; preds = %bb36
  %_100 = load i32, ptr %ek, align 4
  %_99 = icmp eq i32 %_100, 19
  br i1 %_99, label %bb39, label %bb40

bb37:                                             ; preds = %bb36
  store i8 18, ptr %1, align 1
  br label %bb83

bb40:                                             ; preds = %bb38
  %_105 = load i32, ptr %ek, align 4
  %_104 = icmp eq i32 %_105, 20
  br i1 %_104, label %bb41, label %bb42

bb39:                                             ; preds = %bb38
  store i8 19, ptr %1, align 1
  br label %bb83

bb42:                                             ; preds = %bb40
  %_110 = load i32, ptr %ek, align 4
  %_109 = icmp eq i32 %_110, 21
  br i1 %_109, label %bb43, label %bb44

bb41:                                             ; preds = %bb40
  store i8 20, ptr %1, align 1
  br label %bb83

bb44:                                             ; preds = %bb42
  %_115 = load i32, ptr %ek, align 4
  %_114 = icmp eq i32 %_115, 22
  br i1 %_114, label %bb45, label %bb46

bb43:                                             ; preds = %bb42
  store i8 21, ptr %1, align 1
  br label %bb83

bb46:                                             ; preds = %bb44
  %_120 = load i32, ptr %ek, align 4
  %_119 = icmp eq i32 %_120, 23
  br i1 %_119, label %bb47, label %bb48

bb45:                                             ; preds = %bb44
  store i8 22, ptr %1, align 1
  br label %bb83

bb48:                                             ; preds = %bb46
  %_125 = load i32, ptr %ek, align 4
  %_124 = icmp eq i32 %_125, 24
  br i1 %_124, label %bb49, label %bb50

bb47:                                             ; preds = %bb46
  store i8 23, ptr %1, align 1
  br label %bb83

bb50:                                             ; preds = %bb48
  %_130 = load i32, ptr %ek, align 4
  %_129 = icmp eq i32 %_130, 25
  br i1 %_129, label %bb51, label %bb52

bb49:                                             ; preds = %bb48
  store i8 24, ptr %1, align 1
  br label %bb83

bb52:                                             ; preds = %bb50
  %_135 = load i32, ptr %ek, align 4
  %_134 = icmp eq i32 %_135, 26
  br i1 %_134, label %bb53, label %bb54

bb51:                                             ; preds = %bb50
  store i8 25, ptr %1, align 1
  br label %bb83

bb54:                                             ; preds = %bb52
  %_140 = load i32, ptr %ek, align 4
  %_139 = icmp eq i32 %_140, 27
  br i1 %_139, label %bb55, label %bb56

bb53:                                             ; preds = %bb52
  store i8 26, ptr %1, align 1
  br label %bb83

bb56:                                             ; preds = %bb54
  %_145 = load i32, ptr %ek, align 4
  %_144 = icmp eq i32 %_145, 28
  br i1 %_144, label %bb57, label %bb58

bb55:                                             ; preds = %bb54
  store i8 27, ptr %1, align 1
  br label %bb83

bb58:                                             ; preds = %bb56
  %_150 = load i32, ptr %ek, align 4
  %_149 = icmp eq i32 %_150, 29
  br i1 %_149, label %bb59, label %bb60

bb57:                                             ; preds = %bb56
  store i8 28, ptr %1, align 1
  br label %bb83

bb60:                                             ; preds = %bb58
  %_155 = load i32, ptr %ek, align 4
  %_154 = icmp eq i32 %_155, 30
  br i1 %_154, label %bb61, label %bb62

bb59:                                             ; preds = %bb58
  store i8 29, ptr %1, align 1
  br label %bb83

bb62:                                             ; preds = %bb60
  %_160 = load i32, ptr %ek, align 4
  %_159 = icmp eq i32 %_160, 31
  br i1 %_159, label %bb63, label %bb64

bb61:                                             ; preds = %bb60
  store i8 30, ptr %1, align 1
  br label %bb83

bb64:                                             ; preds = %bb62
  %_165 = load i32, ptr %ek, align 4
  %_164 = icmp eq i32 %_165, 32
  br i1 %_164, label %bb65, label %bb66

bb63:                                             ; preds = %bb62
  store i8 31, ptr %1, align 1
  br label %bb83

bb66:                                             ; preds = %bb64
  %_170 = load i32, ptr %ek, align 4
  %_169 = icmp eq i32 %_170, 33
  br i1 %_169, label %bb67, label %bb68

bb65:                                             ; preds = %bb64
  store i8 32, ptr %1, align 1
  br label %bb83

bb68:                                             ; preds = %bb66
  %_175 = load i32, ptr %ek, align 4
  %_174 = icmp eq i32 %_175, 34
  br i1 %_174, label %bb69, label %bb70

bb67:                                             ; preds = %bb66
  store i8 33, ptr %1, align 1
  br label %bb83

bb70:                                             ; preds = %bb68
  %_180 = load i32, ptr %ek, align 4
  %_179 = icmp eq i32 %_180, 35
  br i1 %_179, label %bb71, label %bb72

bb69:                                             ; preds = %bb68
  store i8 34, ptr %1, align 1
  br label %bb83

bb72:                                             ; preds = %bb70
  %_185 = load i32, ptr %ek, align 4
  %_184 = icmp eq i32 %_185, 39
  br i1 %_184, label %bb73, label %bb74

bb71:                                             ; preds = %bb70
  store i8 35, ptr %1, align 1
  br label %bb83

bb74:                                             ; preds = %bb72
  %_190 = load i32, ptr %ek, align 4
  %_189 = icmp eq i32 %_190, 37
  br i1 %_189, label %bb75, label %bb76

bb73:                                             ; preds = %bb72
  store i8 39, ptr %1, align 1
  br label %bb83

bb76:                                             ; preds = %bb74
  %_195 = load i32, ptr %ek, align 4
  %_194 = icmp eq i32 %_195, 36
  br i1 %_194, label %bb77, label %bb78

bb75:                                             ; preds = %bb74
  store i8 37, ptr %1, align 1
  br label %bb83

bb78:                                             ; preds = %bb76
  %_200 = load i32, ptr %ek, align 4
  %_199 = icmp eq i32 %_200, 38
  br i1 %_199, label %bb79, label %bb80

bb77:                                             ; preds = %bb76
  store i8 36, ptr %1, align 1
  br label %bb83

bb80:                                             ; preds = %bb78
  %_205 = load i32, ptr %ek, align 4
  %_204 = icmp eq i32 %_205, 40
  br i1 %_204, label %bb81, label %bb82

bb79:                                             ; preds = %bb78
  store i8 38, ptr %1, align 1
  br label %bb83

bb82:                                             ; preds = %bb80
  store i8 41, ptr %1, align 1
  br label %bb83

bb81:                                             ; preds = %bb80
  store i8 40, ptr %1, align 1
  br label %bb83
}

; std::rt::lang_start
; Function Attrs: nonlazybind uwtable
define hidden i64 @_ZN3std2rt10lang_start17hd4c35bbfd2a30fb8E(ptr %main, i64 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #2 {
start:
  %_9 = alloca ptr, align 8
  %_5 = alloca i64, align 8
  store ptr %main, ptr %_9, align 8
; call std::rt::lang_start_internal
  %0 = call i64 @_ZN3std2rt19lang_start_internal17h79190e3a877a769dE(ptr align 1 %_9, ptr align 8 @vtable.0, i64 %argc, ptr %argv, i8 %sigpipe)
  store i64 %0, ptr %_5, align 8
  %v = load i64, ptr %_5, align 8
  ret i64 %v
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf7706a4bc99c980bE"(ptr align 8 %_1) unnamed_addr #0 {
start:
  %self = alloca i8, align 1
  %_4 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h5e897f3271c6a419E(ptr %_4)
; call <() as std::process::Termination>::report
  %0 = call i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h9f26ae6a3323b466E"()
  store i8 %0, ptr %self, align 1
  %_6 = load i8, ptr %self, align 1
  %1 = zext i8 %_6 to i32
  ret i32 %1
}

; core::fmt::ArgumentV1::new_debug
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, ptr } @_ZN4core3fmt10ArgumentV19new_debug17h1373fa05f9187e2bE(ptr align 4 %x) unnamed_addr #0 {
start:
  %0 = alloca ptr, align 8
  %1 = alloca ptr, align 8
  %2 = alloca { ptr, ptr }, align 8
  store ptr @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17h8884c40909a0eec2E", ptr %1, align 8
  %_4 = load ptr, ptr %1, align 8, !nonnull !3, !noundef !3
  store ptr %x, ptr %0, align 8
  %_6 = load ptr, ptr %0, align 8, !nonnull !3, !align !9, !noundef !3
  store ptr %_6, ptr %2, align 8
  %3 = getelementptr inbounds { ptr, ptr }, ptr %2, i32 0, i32 1
  store ptr %_4, ptr %3, align 8
  %4 = getelementptr inbounds { ptr, ptr }, ptr %2, i32 0, i32 0
  %5 = load ptr, ptr %4, align 8, !nonnull !3, !align !9, !noundef !3
  %6 = getelementptr inbounds { ptr, ptr }, ptr %2, i32 0, i32 1
  %7 = load ptr, ptr %6, align 8, !nonnull !3, !noundef !3
  %8 = insertvalue { ptr, ptr } undef, ptr %5, 0
  %9 = insertvalue { ptr, ptr } %8, ptr %7, 1
  ret { ptr, ptr } %9
}

; core::fmt::ArgumentV1::new_debug
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, ptr } @_ZN4core3fmt10ArgumentV19new_debug17ha7236a35225c120eE(ptr align 8 %x) unnamed_addr #0 {
start:
  %0 = alloca ptr, align 8
  %1 = alloca ptr, align 8
  %2 = alloca { ptr, ptr }, align 8
  store ptr @"_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17ha630232f76cdf57dE", ptr %1, align 8
  %_4 = load ptr, ptr %1, align 8, !nonnull !3, !noundef !3
  store ptr %x, ptr %0, align 8
  %_6 = load ptr, ptr %0, align 8, !nonnull !3, !align !9, !noundef !3
  store ptr %_6, ptr %2, align 8
  %3 = getelementptr inbounds { ptr, ptr }, ptr %2, i32 0, i32 1
  store ptr %_4, ptr %3, align 8
  %4 = getelementptr inbounds { ptr, ptr }, ptr %2, i32 0, i32 0
  %5 = load ptr, ptr %4, align 8, !nonnull !3, !align !9, !noundef !3
  %6 = getelementptr inbounds { ptr, ptr }, ptr %2, i32 0, i32 1
  %7 = load ptr, ptr %6, align 8, !nonnull !3, !noundef !3
  %8 = insertvalue { ptr, ptr } undef, ptr %5, 0
  %9 = insertvalue { ptr, ptr } %8, ptr %7, 1
  ret { ptr, ptr } %9
}

; core::fmt::num::<impl core::fmt::Debug for i32>::fmt
; Function Attrs: inlinehint nonlazybind uwtable
define internal zeroext i1 @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17h8884c40909a0eec2E"(ptr align 4 %self, ptr align 8 %f) unnamed_addr #0 {
start:
  %0 = alloca i8, align 1
; call core::fmt::Formatter::debug_lower_hex
  %_3 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17hacc814ad62bccd75E(ptr align 8 %f)
  br i1 %_3, label %bb2, label %bb4

bb4:                                              ; preds = %start
; call core::fmt::Formatter::debug_upper_hex
  %_7 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17h1de7f6350f80b832E(ptr align 8 %f)
  br i1 %_7, label %bb6, label %bb8

bb2:                                              ; preds = %start
; call core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt
  %1 = call zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h39ae24742179d814E"(ptr align 4 %self, ptr align 8 %f)
  %2 = zext i1 %1 to i8
  store i8 %2, ptr %0, align 1
  br label %bb11

bb11:                                             ; preds = %bb10, %bb2
  %3 = load i8, ptr %0, align 1, !range !6, !noundef !3
  %4 = trunc i8 %3 to i1
  ret i1 %4

bb8:                                              ; preds = %bb4
; call core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt
  %5 = call zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h074201b7268d1f5fE"(ptr align 4 %self, ptr align 8 %f)
  %6 = zext i1 %5 to i8
  store i8 %6, ptr %0, align 1
  br label %bb10

bb6:                                              ; preds = %bb4
; call core::fmt::num::<impl core::fmt::UpperHex for i32>::fmt
  %7 = call zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17h5f0ef384b379ca73E"(ptr align 4 %self, ptr align 8 %f)
  %8 = zext i1 %7 to i8
  store i8 %8, ptr %0, align 1
  br label %bb10

bb10:                                             ; preds = %bb8, %bb6
  br label %bb11
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117h052a350a7920a495E(ptr sret(%"core::fmt::Arguments<'_>") %0, ptr align 8 %pieces.0, i64 %pieces.1, ptr align 8 %args.0, i64 %args.1) unnamed_addr #0 {
start:
  %_24 = alloca { ptr, i64 }, align 8
  %_16 = alloca %"core::fmt::Arguments<'_>", align 8
  %_3 = alloca i8, align 1
  %_4 = icmp ult i64 %pieces.1, %args.1
  br i1 %_4, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %_12 = add i64 %args.1, 1
  %_9 = icmp ugt i64 %pieces.1, %_12
  %1 = zext i1 %_9 to i8
  store i8 %1, ptr %_3, align 1
  br label %bb3

bb1:                                              ; preds = %start
  store i8 1, ptr %_3, align 1
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  %2 = load i8, ptr %_3, align 1, !range !6, !noundef !3
  %3 = trunc i8 %2 to i1
  br i1 %3, label %bb4, label %bb6

bb6:                                              ; preds = %bb3
  store ptr null, ptr %_24, align 8
  %4 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 1
  %5 = getelementptr inbounds { ptr, i64 }, ptr %4, i32 0, i32 0
  store ptr %pieces.0, ptr %5, align 8
  %6 = getelementptr inbounds { ptr, i64 }, ptr %4, i32 0, i32 1
  store i64 %pieces.1, ptr %6, align 8
  %7 = getelementptr inbounds { ptr, i64 }, ptr %_24, i32 0, i32 0
  %8 = load ptr, ptr %7, align 8, !align !7
  %9 = getelementptr inbounds { ptr, i64 }, ptr %_24, i32 0, i32 1
  %10 = load i64, ptr %9, align 8
  %11 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  store ptr %8, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  store i64 %10, ptr %12, align 8
  %13 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 2
  %14 = getelementptr inbounds { ptr, i64 }, ptr %13, i32 0, i32 0
  store ptr %args.0, ptr %14, align 8
  %15 = getelementptr inbounds { ptr, i64 }, ptr %13, i32 0, i32 1
  store i64 %args.1, ptr %15, align 8
  ret void

bb4:                                              ; preds = %bb3
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h052a350a7920a495E(ptr sret(%"core::fmt::Arguments<'_>") %_16, ptr align 8 @alloc25, i64 1, ptr align 8 @alloc13, i64 0)
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hfd9e949092070b66E(ptr %_16, ptr align 8 @alloc129) #11
  unreachable
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h96d407bc8986291dE"(ptr %_1) unnamed_addr #0 {
start:
  %_2 = alloca {}, align 1
  %0 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call core::ops::function::FnOnce::call_once
  %1 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h8ee9325f31e60691E(ptr %0)
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h8ee9325f31e60691E(ptr %0) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %1 = alloca { ptr, i32 }, align 8
  %_2 = alloca {}, align 1
  %_1 = alloca ptr, align 8
  store ptr %0, ptr %_1, align 8
; invoke std::rt::lang_start::{{closure}}
  %2 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hf7706a4bc99c980bE"(ptr align 8 %_1)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  %3 = load ptr, ptr %1, align 8
  %4 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  %5 = load i32, ptr %4, align 8
  %6 = insertvalue { ptr, i32 } undef, ptr %3, 0
  %7 = insertvalue { ptr, i32 } %6, i32 %5, 1
  resume { ptr, i32 } %7

cleanup:                                          ; preds = %start
  %8 = landingpad { ptr, i32 }
          cleanup
  %9 = extractvalue { ptr, i32 } %8, 0
  %10 = extractvalue { ptr, i32 } %8, 1
  %11 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 0
  store ptr %9, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  store i32 %10, ptr %12, align 8
  br label %bb3

bb1:                                              ; preds = %start
  ret i32 %2
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17ha8340e8701cd160aE(ptr %_1) unnamed_addr #0 {
start:
  %_2 = alloca {}, align 1
  call void %_1()
  ret void
}

; core::ptr::drop_in_place<std::io::error::ErrorData<alloc::boxed::Box<std::io::error::Custom>>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr101drop_in_place$LT$std..io..error..ErrorData$LT$alloc..boxed..Box$LT$std..io..error..Custom$GT$$GT$$GT$17he6dbf00caff3c60eE"(ptr %_1) unnamed_addr #2 {
start:
  %0 = load i8, ptr %_1, align 8, !range !10, !noundef !3
  %_2 = zext i8 %0 to i64
  switch i64 %_2, label %bb2 [
    i64 0, label %bb1
    i64 1, label %bb1
    i64 2, label %bb1
  ]

bb2:                                              ; preds = %start
  %1 = getelementptr inbounds %"std::io::error::ErrorData<alloc::boxed::Box<std::io::error::Custom>>::Custom", ptr %_1, i32 0, i32 1
; call core::ptr::drop_in_place<alloc::boxed::Box<std::io::error::Custom>>
  call void @"_ZN4core3ptr68drop_in_place$LT$alloc..boxed..Box$LT$std..io..error..Custom$GT$$GT$17h52dfb61b66beb44dE"(ptr %1)
  br label %bb1

bb1:                                              ; preds = %bb2, %start, %start, %start
  ret void
}

; core::ptr::drop_in_place<alloc::boxed::Box<dyn core::error::Error+core::marker::Send+core::marker::Sync>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr118drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$17h10e20d401bea5b36E"(ptr %_1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %0 = alloca { ptr, i32 }, align 8
  %1 = getelementptr inbounds { ptr, ptr }, ptr %_1, i32 0, i32 0
  %_4.0 = load ptr, ptr %1, align 8
  %2 = getelementptr inbounds { ptr, ptr }, ptr %_1, i32 0, i32 1
  %_4.1 = load ptr, ptr %2, align 8, !nonnull !3, !align !7, !noundef !3
  %3 = getelementptr inbounds ptr, ptr %_4.1, i64 0
  %4 = load ptr, ptr %3, align 8, !invariant.load !3, !nonnull !3
  invoke void %4(ptr %_4.0)
          to label %bb3 unwind label %cleanup

bb4:                                              ; preds = %cleanup
  %5 = getelementptr inbounds { ptr, ptr }, ptr %_1, i32 0, i32 0
  %6 = load ptr, ptr %5, align 8, !nonnull !3, !noundef !3
  %7 = getelementptr inbounds { ptr, ptr }, ptr %_1, i32 0, i32 1
  %8 = load ptr, ptr %7, align 8, !nonnull !3, !align !7, !noundef !3
; invoke alloc::alloc::box_free
  invoke void @_ZN5alloc5alloc8box_free17h5d658757c7deb0e5E(ptr %6, ptr align 8 %8) #12
          to label %bb2 unwind label %abort

cleanup:                                          ; preds = %start
  %9 = landingpad { ptr, i32 }
          cleanup
  %10 = extractvalue { ptr, i32 } %9, 0
  %11 = extractvalue { ptr, i32 } %9, 1
  %12 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0
  store ptr %10, ptr %12, align 8
  %13 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  store i32 %11, ptr %13, align 8
  br label %bb4

bb3:                                              ; preds = %start
  %14 = getelementptr inbounds { ptr, ptr }, ptr %_1, i32 0, i32 0
  %15 = load ptr, ptr %14, align 8, !nonnull !3, !noundef !3
  %16 = getelementptr inbounds { ptr, ptr }, ptr %_1, i32 0, i32 1
  %17 = load ptr, ptr %16, align 8, !nonnull !3, !align !7, !noundef !3
; call alloc::alloc::box_free
  call void @_ZN5alloc5alloc8box_free17h5d658757c7deb0e5E(ptr %15, ptr align 8 %17)
  ret void

abort:                                            ; preds = %bb4
  %18 = landingpad { ptr, i32 }
          cleanup
; call core::panicking::panic_no_unwind
  call void @_ZN4core9panicking15panic_no_unwind17hb07180a78460155fE() #13
  unreachable

bb2:                                              ; preds = %bb4
  %19 = load ptr, ptr %0, align 8
  %20 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  %21 = load i32, ptr %20, align 8
  %22 = insertvalue { ptr, i32 } undef, ptr %19, 0
  %23 = insertvalue { ptr, i32 } %22, i32 %21, 1
  resume { ptr, i32 } %23
}

; core::ptr::drop_in_place<alloc::string::String>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hb63df47af4d3dd7aE"(ptr %_1) unnamed_addr #2 {
start:
; call core::ptr::drop_in_place<alloc::vec::Vec<u8>>
  call void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h178eee755eb7a44dE"(ptr %_1)
  ret void
}

; core::ptr::drop_in_place<std::io::error::Error>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h16f704f8a58dbcb2E"(ptr %_1) unnamed_addr #2 {
start:
; call core::ptr::drop_in_place<std::io::error::repr_bitpacked::Repr>
  call void @"_ZN4core3ptr57drop_in_place$LT$std..io..error..repr_bitpacked..Repr$GT$17h19611b906e604b2eE"(ptr %_1)
  ret void
}

; core::ptr::drop_in_place<std::io::error::Custom>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr43drop_in_place$LT$std..io..error..Custom$GT$17h7c081e28cf5bf90cE"(ptr %_1) unnamed_addr #2 {
start:
; call core::ptr::drop_in_place<alloc::boxed::Box<dyn core::error::Error+core::marker::Send+core::marker::Sync>>
  call void @"_ZN4core3ptr118drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$17h10e20d401bea5b36E"(ptr %_1)
  ret void
}

; core::ptr::drop_in_place<alloc::vec::Vec<u8>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h178eee755eb7a44dE"(ptr %_1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %0 = alloca { ptr, i32 }, align 8
; invoke <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
  invoke void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h0ec7e9ec7b09a065E"(ptr align 8 %_1)
          to label %bb4 unwind label %cleanup

bb3:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  invoke void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h9c4de075de332d34E"(ptr %_1) #12
          to label %bb1 unwind label %abort

cleanup:                                          ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = extractvalue { ptr, i32 } %1, 0
  %3 = extractvalue { ptr, i32 } %1, 1
  %4 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0
  store ptr %2, ptr %4, align 8
  %5 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  store i32 %3, ptr %5, align 8
  br label %bb3

bb4:                                              ; preds = %start
; call core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  call void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h9c4de075de332d34E"(ptr %_1)
  ret void

abort:                                            ; preds = %bb3
  %6 = landingpad { ptr, i32 }
          cleanup
; call core::panicking::panic_no_unwind
  call void @_ZN4core9panicking15panic_no_unwind17hb07180a78460155fE() #13
  unreachable

bb1:                                              ; preds = %bb3
  %7 = load ptr, ptr %0, align 8
  %8 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  %9 = load i32, ptr %8, align 8
  %10 = insertvalue { ptr, i32 } undef, ptr %7, 0
  %11 = insertvalue { ptr, i32 } %10, i32 %9, 1
  resume { ptr, i32 } %11
}

; core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h9c4de075de332d34E"(ptr %_1) unnamed_addr #2 {
start:
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17heb23f6054fdd56caE"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<std::io::error::repr_bitpacked::Repr>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr57drop_in_place$LT$std..io..error..repr_bitpacked..Repr$GT$17h19611b906e604b2eE"(ptr %_1) unnamed_addr #2 {
start:
; call <std::io::error::repr_bitpacked::Repr as core::ops::drop::Drop>::drop
  call void @"_ZN78_$LT$std..io..error..repr_bitpacked..Repr$u20$as$u20$core..ops..drop..Drop$GT$4drop17h7a6d7bfcec218f03E"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<alloc::boxed::Box<std::io::error::Custom>>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr68drop_in_place$LT$alloc..boxed..Box$LT$std..io..error..Custom$GT$$GT$17h52dfb61b66beb44dE"(ptr %_1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %0 = alloca { ptr, i32 }, align 8
  %_4 = load ptr, ptr %_1, align 8
; invoke core::ptr::drop_in_place<std::io::error::Custom>
  invoke void @"_ZN4core3ptr43drop_in_place$LT$std..io..error..Custom$GT$17h7c081e28cf5bf90cE"(ptr %_4)
          to label %bb3 unwind label %cleanup

bb4:                                              ; preds = %cleanup
  %1 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; invoke alloc::alloc::box_free
  invoke void @_ZN5alloc5alloc8box_free17h1e5c21d77960c294E(ptr %1) #12
          to label %bb2 unwind label %abort

cleanup:                                          ; preds = %start
  %2 = landingpad { ptr, i32 }
          cleanup
  %3 = extractvalue { ptr, i32 } %2, 0
  %4 = extractvalue { ptr, i32 } %2, 1
  %5 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0
  store ptr %3, ptr %5, align 8
  %6 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  store i32 %4, ptr %6, align 8
  br label %bb4

bb3:                                              ; preds = %start
  %7 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call alloc::alloc::box_free
  call void @_ZN5alloc5alloc8box_free17h1e5c21d77960c294E(ptr %7)
  ret void

abort:                                            ; preds = %bb4
  %8 = landingpad { ptr, i32 }
          cleanup
; call core::panicking::panic_no_unwind
  call void @_ZN4core9panicking15panic_no_unwind17hb07180a78460155fE() #13
  unreachable

bb2:                                              ; preds = %bb4
  %9 = load ptr, ptr %0, align 8
  %10 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  %11 = load i32, ptr %10, align 8
  %12 = insertvalue { ptr, i32 } undef, ptr %9, 0
  %13 = insertvalue { ptr, i32 } %12, i32 %11, 1
  resume { ptr, i32 } %13
}

; core::ptr::mut_ptr::<impl *mut T>::guaranteed_eq
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17h24c9554e712ecfeeE"(ptr %self, ptr %other) unnamed_addr #0 {
start:
  %0 = alloca i8, align 1
  %1 = alloca i8, align 1
  %2 = icmp eq ptr %self, %other
  %3 = zext i1 %2 to i8
  store i8 %3, ptr %0, align 1
  %_8 = load i8, ptr %0, align 1
  %4 = icmp eq i8 %_8, 2
  br i1 %4, label %bb3, label %bb2

bb3:                                              ; preds = %start
  store i8 2, ptr %1, align 1
  br label %bb4

bb2:                                              ; preds = %start
  %_14 = icmp eq i8 %_8, 1
  %5 = zext i1 %_14 to i8
  store i8 %5, ptr %1, align 1
  br label %bb4

bb4:                                              ; preds = %bb3, %bb2
  %6 = load i8, ptr %1, align 1, !range !11, !noundef !3
  ret i8 %6
}

; core::ptr::mut_ptr::<impl *mut T>::with_metadata_of
; Function Attrs: inlinehint nonlazybind uwtable
define internal ptr @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$16with_metadata_of17hecc9aa2ec60c4821E"(ptr %self, ptr %meta) unnamed_addr #0 {
start:
  %_10 = alloca %"core::ptr::metadata::PtrComponents<()>", align 8
  %_9 = alloca %"core::ptr::metadata::PtrRepr<()>", align 8
  %_7 = alloca %"core::ptr::metadata::PtrRepr<()>", align 8
  store ptr %meta, ptr %_7, align 8
  store ptr %self, ptr %_10, align 8
  %0 = getelementptr inbounds %"core::ptr::metadata::PtrComponents<()>", ptr %_10, i32 0, i32 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_9, ptr align 8 %_10, i64 8, i1 false)
  %1 = load ptr, ptr %_9, align 8
  ret ptr %1
}

; core::ptr::mut_ptr::<impl *mut T>::is_null
; Function Attrs: inlinehint nonlazybind uwtable
define internal zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hada0130d88342330E"(ptr %self) unnamed_addr #0 {
start:
  %0 = alloca ptr, align 8
  %_13 = alloca %"core::ptr::metadata::PtrComponents<u8>", align 8
  %_12 = alloca %"core::ptr::metadata::PtrRepr<u8>", align 8
  %_2 = alloca i8, align 1
  %1 = alloca i8, align 1
  store i64 0, ptr %0, align 8
  %data_address = load ptr, ptr %0, align 8
  store ptr %data_address, ptr %_13, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_12, ptr align 8 %_13, i64 8, i1 false)
  %_5 = load ptr, ptr %_12, align 8
; call core::ptr::mut_ptr::<impl *mut T>::guaranteed_eq
  %2 = call i8 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17h24c9554e712ecfeeE"(ptr %self, ptr %_5), !range !11
  store i8 %2, ptr %_2, align 1
  %3 = load i8, ptr %_2, align 1, !range !11, !noundef !3
  %4 = icmp eq i8 %3, 2
  %_6 = select i1 %4, i64 0, i64 1
  %5 = icmp eq i64 %_6, 0
  br i1 %5, label %bb4, label %bb2

bb4:                                              ; preds = %start
  store i8 0, ptr %1, align 1
  br label %bb5

bb2:                                              ; preds = %start
  %6 = load i8, ptr %_2, align 1, !range !6, !noundef !3
  %res = trunc i8 %6 to i1
  %7 = zext i1 %res to i8
  store i8 %7, ptr %1, align 1
  br label %bb5

bb3:                                              ; No predecessors!
  unreachable

bb5:                                              ; preds = %bb4, %bb2
  %8 = load i8, ptr %1, align 1, !range !6, !noundef !3
  %9 = trunc i8 %8 to i1
  ret i1 %9
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17he8fd658a00d261a8E"(ptr %_1) unnamed_addr #0 {
start:
  ret void
}

; core::ptr::drop_in_place<dyn core::error::Error+core::marker::Send+core::marker::Sync>
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN4core3ptr93drop_in_place$LT$dyn$u20$core..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$17h4e85aacf67576f90E"(ptr %_1.0, ptr align 8 %_1.1) unnamed_addr #2 {
start:
  %0 = getelementptr inbounds ptr, ptr %_1.1, i64 0
  %1 = load ptr, ptr %0, align 8, !invariant.load !3, !nonnull !3
  call void %1(ptr %_1.0)
  ret void
}

; core::hint::unreachable_unchecked
; Function Attrs: inlinehint noreturn nonlazybind uwtable
define internal void @_ZN4core4hint21unreachable_unchecked17h19f9a20c320cdb73E() unnamed_addr #3 {
start:
  unreachable
}

; core::alloc::layout::Layout::array::inner
; Function Attrs: inlinehint nonlazybind uwtable
define internal { i64, i64 } @_ZN4core5alloc6layout6Layout5array5inner17hc7a4ab925aaa4e33E(i64 %element_size, i64 %align, i64 %n) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %_28 = alloca i64, align 8
  %_24 = alloca i64, align 8
  %_16 = alloca { i64, i64 }, align 8
  %_4 = alloca i8, align 1
  %1 = alloca { i64, i64 }, align 8
  %2 = icmp eq i64 %element_size, 0
  br i1 %2, label %bb1, label %bb2

bb1:                                              ; preds = %start
  store i8 0, ptr %_4, align 1
  br label %bb3

bb2:                                              ; preds = %start
  store i64 %align, ptr %_24, align 8
  %_25 = load i64, ptr %_24, align 8, !range !12, !noundef !3
  %_26 = icmp uge i64 -9223372036854775808, %_25
  call void @llvm.assume(i1 %_26)
  %_27 = icmp ule i64 1, %_25
  call void @llvm.assume(i1 %_27)
  %_21 = sub i64 %_25, 1
  %_9 = sub i64 9223372036854775807, %_21
  %_12 = icmp eq i64 %element_size, 0
  %3 = call i1 @llvm.expect.i1(i1 %_12, i1 false)
  br i1 %3, label %panic, label %bb4

bb4:                                              ; preds = %bb2
  %_8 = udiv i64 %_9, %element_size
  %_6 = icmp ugt i64 %n, %_8
  %4 = zext i1 %_6 to i8
  store i8 %4, ptr %_4, align 1
  br label %bb3

panic:                                            ; preds = %bb2
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h341545107301821dE(ptr align 1 @str.1, i64 25, ptr align 8 @alloc131) #11
  unreachable

bb3:                                              ; preds = %bb1, %bb4
  %5 = load i8, ptr %_4, align 1, !range !6, !noundef !3
  %6 = trunc i8 %5 to i1
  br i1 %6, label %bb5, label %bb6

bb6:                                              ; preds = %bb3
  %array_size = mul i64 %element_size, %n
  store i64 %align, ptr %_28, align 8
  %_29 = load i64, ptr %_28, align 8, !range !12, !noundef !3
  %_30 = icmp uge i64 -9223372036854775808, %_29
  call void @llvm.assume(i1 %_30)
  %_31 = icmp ule i64 1, %_29
  call void @llvm.assume(i1 %_31)
  store i64 %_29, ptr %0, align 8
  %_33 = load i64, ptr %0, align 8, !range !12, !noundef !3
  store i64 %array_size, ptr %_16, align 8
  %7 = getelementptr inbounds { i64, i64 }, ptr %_16, i32 0, i32 1
  store i64 %_33, ptr %7, align 8
  %8 = getelementptr inbounds { i64, i64 }, ptr %_16, i32 0, i32 0
  %9 = load i64, ptr %8, align 8
  %10 = getelementptr inbounds { i64, i64 }, ptr %_16, i32 0, i32 1
  %11 = load i64, ptr %10, align 8, !range !12, !noundef !3
  %12 = getelementptr inbounds { i64, i64 }, ptr %1, i32 0, i32 0
  store i64 %9, ptr %12, align 8
  %13 = getelementptr inbounds { i64, i64 }, ptr %1, i32 0, i32 1
  store i64 %11, ptr %13, align 8
  br label %bb7

bb5:                                              ; preds = %bb3
  %14 = getelementptr inbounds { i64, i64 }, ptr %1, i32 0, i32 1
  store i64 0, ptr %14, align 8
  br label %bb7

bb7:                                              ; preds = %bb6, %bb5
  %15 = getelementptr inbounds { i64, i64 }, ptr %1, i32 0, i32 0
  %16 = load i64, ptr %15, align 8
  %17 = getelementptr inbounds { i64, i64 }, ptr %1, i32 0, i32 1
  %18 = load i64, ptr %17, align 8, !range !13, !noundef !3
  %19 = insertvalue { i64, i64 } undef, i64 %16, 0
  %20 = insertvalue { i64, i64 } %19, i64 %18, 1
  ret { i64, i64 } %20
}

; core::result::Result<T,E>::unwrap_unchecked
; Function Attrs: inlinehint nonlazybind uwtable
define internal { i64, i64 } @"_ZN4core6result19Result$LT$T$C$E$GT$16unwrap_unchecked17h01820e89d885a00dE"(i64 %0, i64 %1, ptr align 8 %2) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %3 = alloca { ptr, i32 }, align 8
  %self = alloca { i64, i64 }, align 8
  %4 = getelementptr inbounds { i64, i64 }, ptr %self, i32 0, i32 0
  store i64 %0, ptr %4, align 8
  %5 = getelementptr inbounds { i64, i64 }, ptr %self, i32 0, i32 1
  store i64 %1, ptr %5, align 8
  %6 = getelementptr inbounds { i64, i64 }, ptr %self, i32 0, i32 1
  %7 = load i64, ptr %6, align 8, !range !13, !noundef !3
  %8 = icmp eq i64 %7, 0
  %_3 = select i1 %8, i64 1, i64 0
  %9 = icmp eq i64 %_3, 0
  br i1 %9, label %bb3, label %bb1

bb3:                                              ; preds = %start
  %10 = getelementptr inbounds { i64, i64 }, ptr %self, i32 0, i32 0
  %t.0 = load i64, ptr %10, align 8
  %11 = getelementptr inbounds { i64, i64 }, ptr %self, i32 0, i32 1
  %t.1 = load i64, ptr %11, align 8, !range !12, !noundef !3
  %12 = getelementptr inbounds { i64, i64 }, ptr %self, i32 0, i32 1
  %13 = load i64, ptr %12, align 8, !range !13, !noundef !3
  %14 = icmp eq i64 %13, 0
  %_7 = select i1 %14, i64 1, i64 0
  %15 = icmp eq i64 %_7, 0
  br i1 %15, label %bb5, label %bb6

bb1:                                              ; preds = %start
; invoke core::hint::unreachable_unchecked
  invoke void @_ZN4core4hint21unreachable_unchecked17h19f9a20c320cdb73E() #11
          to label %unreachable unwind label %cleanup

bb2:                                              ; No predecessors!
  unreachable

bb10:                                             ; preds = %cleanup
  %16 = getelementptr inbounds { i64, i64 }, ptr %self, i32 0, i32 1
  %17 = load i64, ptr %16, align 8, !range !13, !noundef !3
  %18 = icmp eq i64 %17, 0
  %_8 = select i1 %18, i64 1, i64 0
  %19 = icmp eq i64 %_8, 0
  br i1 %19, label %bb7, label %bb9

cleanup:                                          ; preds = %bb1
  %20 = landingpad { ptr, i32 }
          cleanup
  %21 = extractvalue { ptr, i32 } %20, 0
  %22 = extractvalue { ptr, i32 } %20, 1
  %23 = getelementptr inbounds { ptr, i32 }, ptr %3, i32 0, i32 0
  store ptr %21, ptr %23, align 8
  %24 = getelementptr inbounds { ptr, i32 }, ptr %3, i32 0, i32 1
  store i32 %22, ptr %24, align 8
  br label %bb10

unreachable:                                      ; preds = %bb1
  unreachable

bb7:                                              ; preds = %bb10
  br i1 true, label %bb8, label %bb4

bb9:                                              ; preds = %bb10
  br label %bb4

bb4:                                              ; preds = %bb8, %bb7, %bb9
  %25 = load ptr, ptr %3, align 8
  %26 = getelementptr inbounds { ptr, i32 }, ptr %3, i32 0, i32 1
  %27 = load i32, ptr %26, align 8
  %28 = insertvalue { ptr, i32 } undef, ptr %25, 0
  %29 = insertvalue { ptr, i32 } %28, i32 %27, 1
  resume { ptr, i32 } %29

bb8:                                              ; preds = %bb7
  br label %bb4

bb5:                                              ; preds = %bb6, %bb3
  %30 = insertvalue { i64, i64 } undef, i64 %t.0, 0
  %31 = insertvalue { i64, i64 } %30, i64 %t.1, 1
  ret { i64, i64 } %31

bb6:                                              ; preds = %bb3
  br label %bb5
}

; core::result::Result<T,E>::unwrap
; Function Attrs: inlinehint nonlazybind uwtable
define internal i64 @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17he320ea76755b89c2E"(ptr %self, ptr align 8 %0) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %1 = alloca { ptr, i32 }, align 8
  %e = alloca ptr, align 8
  %_2 = load i64, ptr %self, align 8, !range !14, !noundef !3
  %2 = icmp eq i64 %_2, 0
  br i1 %2, label %bb3, label %bb1

bb3:                                              ; preds = %start
  %3 = getelementptr inbounds %"core::result::Result<usize, std::io::error::Error>::Ok", ptr %self, i32 0, i32 1
  %t = load i64, ptr %3, align 8
  ret i64 %t

bb1:                                              ; preds = %start
  %4 = getelementptr inbounds %"core::result::Result<usize, std::io::error::Error>::Err", ptr %self, i32 0, i32 1
  %5 = load ptr, ptr %4, align 8, !nonnull !3, !noundef !3
  store ptr %5, ptr %e, align 8
; invoke core::result::unwrap_failed
  invoke void @_ZN4core6result13unwrap_failed17h4d34d8346233eb49E(ptr align 1 @alloc132, i64 43, ptr align 1 %e, ptr align 8 @vtable.2, ptr align 8 %0) #11
          to label %unreachable unwind label %cleanup

bb2:                                              ; No predecessors!
  unreachable

bb4:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<std::io::error::Error>
  invoke void @"_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h16f704f8a58dbcb2E"(ptr %e) #12
          to label %bb5 unwind label %abort

cleanup:                                          ; preds = %bb1
  %6 = landingpad { ptr, i32 }
          cleanup
  %7 = extractvalue { ptr, i32 } %6, 0
  %8 = extractvalue { ptr, i32 } %6, 1
  %9 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 0
  store ptr %7, ptr %9, align 8
  %10 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  store i32 %8, ptr %10, align 8
  br label %bb4

unreachable:                                      ; preds = %bb1
  unreachable

abort:                                            ; preds = %bb4
  %11 = landingpad { ptr, i32 }
          cleanup
; call core::panicking::panic_no_unwind
  call void @_ZN4core9panicking15panic_no_unwind17hb07180a78460155fE() #13
  unreachable

bb5:                                              ; preds = %bb4
  %12 = load ptr, ptr %1, align 8
  %13 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  %14 = load i32, ptr %13, align 8
  %15 = insertvalue { ptr, i32 } undef, ptr %12, 0
  %16 = insertvalue { ptr, i32 } %15, i32 %14, 1
  resume { ptr, i32 } %16
}

; <T as core::convert::Into<U>>::into
; Function Attrs: nonlazybind uwtable
define internal ptr @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17hd9625558e087c6f0E"(ptr %self) unnamed_addr #2 {
start:
; call <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
  %0 = call ptr @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h69c375d0365376d4E"(ptr %self)
  ret ptr %0
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h9f26ae6a3323b466E"() unnamed_addr #0 {
start:
  ret i8 0
}

; <alloc::string::String as core::fmt::Debug>::fmt
; Function Attrs: inlinehint nonlazybind uwtable
define internal zeroext i1 @"_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17ha630232f76cdf57dE"(ptr align 8 %self, ptr align 8 %f) unnamed_addr #0 {
start:
  %0 = alloca { ptr, i64 }, align 8
; call <alloc::vec::Vec<T,A> as core::ops::deref::Deref>::deref
  %1 = call { ptr, i64 } @"_ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h5343ef9ee9382e02E"(ptr align 8 %self)
  %_9.0 = extractvalue { ptr, i64 } %1, 0
  %_9.1 = extractvalue { ptr, i64 } %1, 1
  %2 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  store ptr %_9.0, ptr %2, align 8
  %3 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  store i64 %_9.1, ptr %3, align 8
  %4 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  %_5.0 = load ptr, ptr %4, align 8, !nonnull !3, !align !9, !noundef !3
  %5 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  %_5.1 = load i64, ptr %5, align 8
; call <str as core::fmt::Debug>::fmt
  %6 = call zeroext i1 @"_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17h4c1c0ac714e96665E"(ptr align 1 %_5.0, i64 %_5.1, ptr align 8 %f)
  ret i1 %6
}

; alloc::alloc::box_free
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN5alloc5alloc8box_free17h1e5c21d77960c294E(ptr %0) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %1 = alloca { ptr, i32 }, align 8
  %2 = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %pointer = alloca ptr, align 8
  %unique = alloca ptr, align 8
  %_15 = alloca ptr, align 8
  %layout = alloca { i64, i64 }, align 8
  %alloc = alloca %"alloc::alloc::Global", align 1
  %ptr = alloca ptr, align 8
  store ptr %0, ptr %ptr, align 8
  %self = load ptr, ptr %ptr, align 8, !nonnull !3, !noundef !3
  store i64 24, ptr %4, align 8
  %size = load i64, ptr %4, align 8
  %self1 = load ptr, ptr %ptr, align 8, !nonnull !3, !noundef !3
  store i64 8, ptr %3, align 8
  %align = load i64, ptr %3, align 8
  store i64 %align, ptr %2, align 8
  %_28 = load i64, ptr %2, align 8, !range !12, !noundef !3
  br label %bb7

bb7:                                              ; preds = %start
  store i64 %size, ptr %layout, align 8
  %5 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  store i64 %_28, ptr %5, align 8
  %self2 = load ptr, ptr %ptr, align 8, !nonnull !3, !noundef !3
  store ptr %self2, ptr %pointer, align 8
  %_44 = load ptr, ptr %pointer, align 8, !nonnull !3, !noundef !3
  store ptr %_44, ptr %unique, align 8
  %self3 = load ptr, ptr %unique, align 8, !nonnull !3, !noundef !3
  store ptr %self3, ptr %_15, align 8
  %6 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  %_18.0 = load i64, ptr %6, align 8
  %7 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %_18.1 = load i64, ptr %7, align 8, !range !12, !noundef !3
  %8 = load ptr, ptr %_15, align 8, !nonnull !3, !noundef !3
; invoke <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  invoke void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h70f16c9fd85d0f7aE"(ptr align 1 %alloc, ptr %8, i64 %_18.0, i64 %_18.1)
          to label %bb3 unwind label %cleanup

bb5:                                              ; preds = %cleanup
  %9 = load ptr, ptr %1, align 8
  %10 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  %11 = load i32, ptr %10, align 8
  %12 = insertvalue { ptr, i32 } undef, ptr %9, 0
  %13 = insertvalue { ptr, i32 } %12, i32 %11, 1
  resume { ptr, i32 } %13

cleanup:                                          ; preds = %bb7
  %14 = landingpad { ptr, i32 }
          cleanup
  %15 = extractvalue { ptr, i32 } %14, 0
  %16 = extractvalue { ptr, i32 } %14, 1
  %17 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 0
  store ptr %15, ptr %17, align 8
  %18 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  store i32 %16, ptr %18, align 8
  br label %bb5

bb3:                                              ; preds = %bb7
  ret void
}

; alloc::alloc::box_free
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN5alloc5alloc8box_free17h5d658757c7deb0e5E(ptr %0, ptr align 8 %1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %2 = alloca { ptr, i32 }, align 8
  %3 = alloca i64, align 8
  %4 = alloca i64, align 8
  %5 = alloca i64, align 8
  %pointer = alloca ptr, align 8
  %unique = alloca ptr, align 8
  %_15 = alloca ptr, align 8
  %layout = alloca { i64, i64 }, align 8
  %alloc = alloca %"alloc::alloc::Global", align 1
  %ptr = alloca { ptr, ptr }, align 8
  %6 = getelementptr inbounds { ptr, ptr }, ptr %ptr, i32 0, i32 0
  store ptr %0, ptr %6, align 8
  %7 = getelementptr inbounds { ptr, ptr }, ptr %ptr, i32 0, i32 1
  store ptr %1, ptr %7, align 8
  %8 = getelementptr inbounds { ptr, ptr }, ptr %ptr, i32 0, i32 0
  %self.0 = load ptr, ptr %8, align 8, !nonnull !3, !noundef !3
  %9 = getelementptr inbounds { ptr, ptr }, ptr %ptr, i32 0, i32 1
  %self.1 = load ptr, ptr %9, align 8, !nonnull !3, !align !7, !noundef !3
  %10 = getelementptr inbounds i64, ptr %self.1, i64 1
  %11 = load i64, ptr %10, align 8, !invariant.load !3
  %12 = getelementptr inbounds i64, ptr %self.1, i64 2
  %13 = load i64, ptr %12, align 8, !range !15, !invariant.load !3
  store i64 %11, ptr %5, align 8
  %size = load i64, ptr %5, align 8
  %14 = getelementptr inbounds { ptr, ptr }, ptr %ptr, i32 0, i32 0
  %self.01 = load ptr, ptr %14, align 8, !nonnull !3, !noundef !3
  %15 = getelementptr inbounds { ptr, ptr }, ptr %ptr, i32 0, i32 1
  %self.12 = load ptr, ptr %15, align 8, !nonnull !3, !align !7, !noundef !3
  %16 = getelementptr inbounds i64, ptr %self.12, i64 1
  %17 = load i64, ptr %16, align 8, !invariant.load !3
  %18 = getelementptr inbounds i64, ptr %self.12, i64 2
  %19 = load i64, ptr %18, align 8, !range !15, !invariant.load !3
  store i64 %19, ptr %4, align 8
  %align = load i64, ptr %4, align 8
  store i64 %align, ptr %3, align 8
  %_28 = load i64, ptr %3, align 8, !range !12, !noundef !3
  br label %bb7

bb7:                                              ; preds = %start
  store i64 %size, ptr %layout, align 8
  %20 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  store i64 %_28, ptr %20, align 8
  %21 = getelementptr inbounds { ptr, ptr }, ptr %ptr, i32 0, i32 0
  %self.03 = load ptr, ptr %21, align 8, !nonnull !3, !noundef !3
  %22 = getelementptr inbounds { ptr, ptr }, ptr %ptr, i32 0, i32 1
  %self.14 = load ptr, ptr %22, align 8, !nonnull !3, !align !7, !noundef !3
  store ptr %self.03, ptr %pointer, align 8
  %_44 = load ptr, ptr %pointer, align 8, !nonnull !3, !noundef !3
  store ptr %_44, ptr %unique, align 8
  %self = load ptr, ptr %unique, align 8, !nonnull !3, !noundef !3
  store ptr %self, ptr %_15, align 8
  %23 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  %_18.0 = load i64, ptr %23, align 8
  %24 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %_18.1 = load i64, ptr %24, align 8, !range !12, !noundef !3
  %25 = load ptr, ptr %_15, align 8, !nonnull !3, !noundef !3
; invoke <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  invoke void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h70f16c9fd85d0f7aE"(ptr align 1 %alloc, ptr %25, i64 %_18.0, i64 %_18.1)
          to label %bb3 unwind label %cleanup

bb5:                                              ; preds = %cleanup
  %26 = load ptr, ptr %2, align 8
  %27 = getelementptr inbounds { ptr, i32 }, ptr %2, i32 0, i32 1
  %28 = load i32, ptr %27, align 8
  %29 = insertvalue { ptr, i32 } undef, ptr %26, 0
  %30 = insertvalue { ptr, i32 } %29, i32 %28, 1
  resume { ptr, i32 } %30

cleanup:                                          ; preds = %bb7
  %31 = landingpad { ptr, i32 }
          cleanup
  %32 = extractvalue { ptr, i32 } %31, 0
  %33 = extractvalue { ptr, i32 } %31, 1
  %34 = getelementptr inbounds { ptr, i32 }, ptr %2, i32 0, i32 0
  store ptr %32, ptr %34, align 8
  %35 = getelementptr inbounds { ptr, i32 }, ptr %2, i32 0, i32 1
  store i32 %33, ptr %35, align 8
  br label %bb5

bb3:                                              ; preds = %bb7
  ret void
}

; alloc::boxed::Box<T,A>::from_raw_in
; Function Attrs: inlinehint nonlazybind uwtable
define internal align 8 ptr @"_ZN5alloc5boxed16Box$LT$T$C$A$GT$11from_raw_in17h13599576de766e61E"(ptr %raw) unnamed_addr #0 {
start:
  %_6 = alloca ptr, align 8
  %_3 = alloca ptr, align 8
  %0 = alloca ptr, align 8
  store ptr %raw, ptr %_6, align 8
  %1 = load ptr, ptr %_6, align 8, !nonnull !3, !noundef !3
  store ptr %1, ptr %_3, align 8
  %2 = load ptr, ptr %_3, align 8, !nonnull !3, !noundef !3
  store ptr %2, ptr %0, align 8
  %3 = load ptr, ptr %0, align 8, !nonnull !3, !align !7, !noundef !3
  ret ptr %3
}

; alloc::string::String::new
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_ZN5alloc6string6String3new17h3044a7d4920107efE(ptr sret(%"alloc::string::String") %0) unnamed_addr #0 {
start:
  %_1 = alloca %"alloc::vec::Vec<u8>", align 8
  %_2.0 = load i64, ptr @0, align 8
  %_2.1 = load ptr, ptr getelementptr inbounds ({ i64, ptr }, ptr @0, i32 0, i32 1), align 8, !nonnull !3, !noundef !3
  %1 = getelementptr inbounds { i64, ptr }, ptr %_1, i32 0, i32 0
  store i64 %_2.0, ptr %1, align 8
  %2 = getelementptr inbounds { i64, ptr }, ptr %_1, i32 0, i32 1
  store ptr %_2.1, ptr %2, align 8
  %3 = getelementptr inbounds %"alloc::vec::Vec<u8>", ptr %_1, i32 0, i32 1
  store i64 0, ptr %3, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %0, ptr align 8 %_1, i64 24, i1 false)
  ret void
}

; alloc::raw_vec::RawVec<T,A>::current_memory
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h1701813af16b9ce0E"(ptr sret(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>") %0, ptr align 8 %self) unnamed_addr #2 {
start:
  %1 = alloca i64, align 8
  %pointer = alloca ptr, align 8
  %_10 = alloca ptr, align 8
  %_8 = alloca { ptr, { i64, i64 } }, align 8
  %_2 = alloca i8, align 1
  br i1 false, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %_4 = load i64, ptr %self, align 8
  %_3 = icmp eq i64 %_4, 0
  %2 = zext i1 %_3 to i8
  store i8 %2, ptr %_2, align 1
  br label %bb3

bb1:                                              ; preds = %start
  store i8 1, ptr %_2, align 1
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  %3 = load i8, ptr %_2, align 1, !range !6, !noundef !3
  %4 = trunc i8 %3 to i1
  br i1 %4, label %bb4, label %bb5

bb5:                                              ; preds = %bb3
  %n = load i64, ptr %self, align 8
  store i64 1, ptr %1, align 8
  %_14 = load i64, ptr %1, align 8, !range !12, !noundef !3
; call core::alloc::layout::Layout::array::inner
  %5 = call { i64, i64 } @_ZN4core5alloc6layout6Layout5array5inner17hc7a4ab925aaa4e33E(i64 1, i64 %_14, i64 %n)
  %_6.0 = extractvalue { i64, i64 } %5, 0
  %_6.1 = extractvalue { i64, i64 } %5, 1
; call core::result::Result<T,E>::unwrap_unchecked
  %6 = call { i64, i64 } @"_ZN4core6result19Result$LT$T$C$E$GT$16unwrap_unchecked17h01820e89d885a00dE"(i64 %_6.0, i64 %_6.1, ptr align 8 @alloc137)
  %layout.0 = extractvalue { i64, i64 } %6, 0
  %layout.1 = extractvalue { i64, i64 } %6, 1
  %7 = getelementptr inbounds { i64, ptr }, ptr %self, i32 0, i32 1
  %self1 = load ptr, ptr %7, align 8, !nonnull !3, !noundef !3
  store ptr %self1, ptr %pointer, align 8
  %_31 = load ptr, ptr %pointer, align 8, !nonnull !3, !noundef !3
  store ptr %_31, ptr %_10, align 8
  %8 = load ptr, ptr %_10, align 8, !nonnull !3, !noundef !3
; call <T as core::convert::Into<U>>::into
  %_9 = call ptr @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17hd9625558e087c6f0E"(ptr %8)
  store ptr %_9, ptr %_8, align 8
  %9 = getelementptr inbounds { ptr, { i64, i64 } }, ptr %_8, i32 0, i32 1
  %10 = getelementptr inbounds { i64, i64 }, ptr %9, i32 0, i32 0
  store i64 %layout.0, ptr %10, align 8
  %11 = getelementptr inbounds { i64, i64 }, ptr %9, i32 0, i32 1
  store i64 %layout.1, ptr %11, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %0, ptr align 8 %_8, i64 24, i1 false)
  br label %bb8

bb4:                                              ; preds = %bb3
  %12 = getelementptr inbounds %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>", ptr %0, i32 0, i32 1
  store i64 0, ptr %12, align 8
  br label %bb8

bb8:                                              ; preds = %bb5, %bb4
  ret void
}

; <alloc::alloc::Global as core::alloc::Allocator>::deallocate
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h70f16c9fd85d0f7aE"(ptr align 1 %self, ptr %ptr, i64 %0, i64 %1) unnamed_addr #0 {
start:
  %_16 = alloca i64, align 8
  %layout1 = alloca { i64, i64 }, align 8
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  store i64 %0, ptr %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  store i64 %1, ptr %3, align 8
  %_4 = load i64, ptr %layout, align 8
  %4 = icmp eq i64 %_4, 0
  br i1 %4, label %bb2, label %bb1

bb2:                                              ; preds = %start
  br label %bb3

bb1:                                              ; preds = %start
  %5 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 0
  %6 = load i64, ptr %5, align 8
  %7 = getelementptr inbounds { i64, i64 }, ptr %layout, i32 0, i32 1
  %8 = load i64, ptr %7, align 8, !range !12, !noundef !3
  %9 = getelementptr inbounds { i64, i64 }, ptr %layout1, i32 0, i32 0
  store i64 %6, ptr %9, align 8
  %10 = getelementptr inbounds { i64, i64 }, ptr %layout1, i32 0, i32 1
  store i64 %8, ptr %10, align 8
  %_11 = load i64, ptr %layout1, align 8
  %11 = getelementptr inbounds { i64, i64 }, ptr %layout1, i32 0, i32 1
  %self2 = load i64, ptr %11, align 8, !range !12, !noundef !3
  store i64 %self2, ptr %_16, align 8
  %_17 = load i64, ptr %_16, align 8, !range !12, !noundef !3
  %_18 = icmp uge i64 -9223372036854775808, %_17
  call void @llvm.assume(i1 %_18)
  %_19 = icmp ule i64 1, %_17
  call void @llvm.assume(i1 %_19)
  call void @__rust_dealloc(ptr %ptr, i64 %_11, i64 %_17) #14
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  ret void
}

; <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h0ec7e9ec7b09a065E"(ptr align 8 %self) unnamed_addr #2 {
start:
  %_18 = alloca { ptr, i64 }, align 8
  %_17 = alloca %"core::ptr::metadata::PtrRepr<[u8]>", align 8
  %0 = getelementptr inbounds { i64, ptr }, ptr %self, i32 0, i32 1
  %self1 = load ptr, ptr %0, align 8, !nonnull !3, !noundef !3
; call core::ptr::mut_ptr::<impl *mut T>::is_null
  %_8 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hada0130d88342330E"(ptr %self1)
  %_7 = xor i1 %_8, true
  call void @llvm.assume(i1 %_7)
  %1 = getelementptr inbounds %"alloc::vec::Vec<u8>", ptr %self, i32 0, i32 1
  %len = load i64, ptr %1, align 8
  store ptr %self1, ptr %_18, align 8
  %2 = getelementptr inbounds { ptr, i64 }, ptr %_18, i32 0, i32 1
  store i64 %len, ptr %2, align 8
  %3 = getelementptr inbounds { ptr, i64 }, ptr %_18, i32 0, i32 0
  %4 = load ptr, ptr %3, align 8
  %5 = getelementptr inbounds { ptr, i64 }, ptr %_18, i32 0, i32 1
  %6 = load i64, ptr %5, align 8
  %7 = getelementptr inbounds { ptr, i64 }, ptr %_17, i32 0, i32 0
  store ptr %4, ptr %7, align 8
  %8 = getelementptr inbounds { ptr, i64 }, ptr %_17, i32 0, i32 1
  store i64 %6, ptr %8, align 8
  %9 = getelementptr inbounds { ptr, i64 }, ptr %_17, i32 0, i32 0
  %_2.0 = load ptr, ptr %9, align 8
  %10 = getelementptr inbounds { ptr, i64 }, ptr %_17, i32 0, i32 1
  %_2.1 = load i64, ptr %10, align 8
  ret void
}

; <alloc::vec::Vec<T,A> as core::ops::deref::Deref>::deref
; Function Attrs: inlinehint nonlazybind uwtable
define internal { ptr, i64 } @"_ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h5343ef9ee9382e02E"(ptr align 8 %self) unnamed_addr #0 {
start:
  %_24 = alloca { ptr, i64 }, align 8
  %_23 = alloca %"core::ptr::metadata::PtrRepr<[u8]>", align 8
  %0 = getelementptr inbounds { i64, ptr }, ptr %self, i32 0, i32 1
  %self1 = load ptr, ptr %0, align 8, !nonnull !3, !noundef !3
; call core::ptr::mut_ptr::<impl *mut T>::is_null
  %_8 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hada0130d88342330E"(ptr %self1)
  %_7 = xor i1 %_8, true
  call void @llvm.assume(i1 %_7)
  %1 = getelementptr inbounds %"alloc::vec::Vec<u8>", ptr %self, i32 0, i32 1
  %len = load i64, ptr %1, align 8
  store ptr %self1, ptr %_24, align 8
  %2 = getelementptr inbounds { ptr, i64 }, ptr %_24, i32 0, i32 1
  store i64 %len, ptr %2, align 8
  %3 = getelementptr inbounds { ptr, i64 }, ptr %_24, i32 0, i32 0
  %4 = load ptr, ptr %3, align 8
  %5 = getelementptr inbounds { ptr, i64 }, ptr %_24, i32 0, i32 1
  %6 = load i64, ptr %5, align 8
  %7 = getelementptr inbounds { ptr, i64 }, ptr %_23, i32 0, i32 0
  store ptr %4, ptr %7, align 8
  %8 = getelementptr inbounds { ptr, i64 }, ptr %_23, i32 0, i32 1
  store i64 %6, ptr %8, align 8
  %9 = getelementptr inbounds { ptr, i64 }, ptr %_23, i32 0, i32 0
  %_14.0 = load ptr, ptr %9, align 8
  %10 = getelementptr inbounds { ptr, i64 }, ptr %_23, i32 0, i32 1
  %_14.1 = load i64, ptr %10, align 8
  %11 = insertvalue { ptr, i64 } undef, ptr %_14.0, 0
  %12 = insertvalue { ptr, i64 } %11, i64 %_14.1, 1
  ret { ptr, i64 } %12
}

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nonlazybind uwtable
define internal void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17heb23f6054fdd56caE"(ptr align 8 %self) unnamed_addr #2 {
start:
  %_2 = alloca %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>", align 8
; call alloc::raw_vec::RawVec<T,A>::current_memory
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h1701813af16b9ce0E"(ptr sret(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>") %_2, ptr align 8 %self)
  %0 = getelementptr inbounds %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>", ptr %_2, i32 0, i32 1
  %1 = load i64, ptr %0, align 8, !range !13, !noundef !3
  %2 = icmp eq i64 %1, 0
  %_4 = select i1 %2, i64 0, i64 1
  %3 = icmp eq i64 %_4, 1
  br i1 %3, label %bb2, label %bb4

bb2:                                              ; preds = %start
  %ptr = load ptr, ptr %_2, align 8, !nonnull !3, !noundef !3
  %4 = getelementptr inbounds { ptr, { i64, i64 } }, ptr %_2, i32 0, i32 1
  %5 = getelementptr inbounds { i64, i64 }, ptr %4, i32 0, i32 0
  %layout.0 = load i64, ptr %5, align 8
  %6 = getelementptr inbounds { i64, i64 }, ptr %4, i32 0, i32 1
  %layout.1 = load i64, ptr %6, align 8, !range !12, !noundef !3
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h70f16c9fd85d0f7aE"(ptr align 1 %self, ptr %ptr, i64 %layout.0, i64 %layout.1)
  br label %bb4

bb4:                                              ; preds = %bb2, %start
  ret void
}

; <std::io::error::repr_bitpacked::Repr as core::ops::drop::Drop>::drop
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN78_$LT$std..io..error..repr_bitpacked..Repr$u20$as$u20$core..ops..drop..Drop$GT$4drop17h7a6d7bfcec218f03E"(ptr align 8 %self) unnamed_addr #0 {
start:
  %_2 = alloca %"std::io::error::ErrorData<alloc::boxed::Box<std::io::error::Custom>>", align 8
  %_3 = load ptr, ptr %self, align 8, !nonnull !3, !noundef !3
; call std::io::error::repr_bitpacked::decode_repr
  call void @_ZN3std2io5error14repr_bitpacked11decode_repr17h1e517f8878b0c3b5E(ptr sret(%"std::io::error::ErrorData<alloc::boxed::Box<std::io::error::Custom>>") %_2, ptr %_3)
; call core::ptr::drop_in_place<std::io::error::ErrorData<alloc::boxed::Box<std::io::error::Custom>>>
  call void @"_ZN4core3ptr101drop_in_place$LT$std..io..error..ErrorData$LT$alloc..boxed..Box$LT$std..io..error..Custom$GT$$GT$$GT$17he6dbf00caff3c60eE"(ptr %_2)
  ret void
}

; <std::io::error::repr_bitpacked::Repr as core::ops::drop::Drop>::drop::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal align 8 ptr @"_ZN78_$LT$std..io..error..repr_bitpacked..Repr$u20$as$u20$core..ops..drop..Drop$GT$4drop28_$u7b$$u7b$closure$u7d$$u7d$17h7f1769e47fcd9dc1E"(ptr %p) unnamed_addr #0 {
start:
; call alloc::boxed::Box<T,A>::from_raw_in
  %0 = call align 8 ptr @"_ZN5alloc5boxed16Box$LT$T$C$A$GT$11from_raw_in17h13599576de766e61E"(ptr %p)
  ret ptr %0
}

; f::main
; Function Attrs: nonlazybind uwtable
define internal void @_ZN1f4main17h583fb028a6d62acbE() unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %0 = alloca { ptr, i32 }, align 8
  %_51 = alloca i8, align 1
  %_43 = alloca [1 x { ptr, ptr }], align 8
  %_36 = alloca %"core::fmt::Arguments<'_>", align 8
  %x2 = alloca i32, align 4
  %_29 = alloca [1 x { ptr, ptr }], align 8
  %_22 = alloca %"core::fmt::Arguments<'_>", align 8
  %x = alloca %"alloc::string::String", align 8
  %v1 = alloca %"alloc::string::String", align 8
  %_17 = alloca %"alloc::string::String", align 8
  %v = alloca %"e<alloc::string::String, i32>", align 8
  %_13 = alloca ptr, align 8
  %_11 = alloca %"core::result::Result<usize, std::io::error::Error>", align 8
  %_3 = alloca %"core::fmt::Arguments<'_>", align 8
  %line = alloca %"alloc::string::String", align 8
  store i8 0, ptr %_51, align 1
  store i8 1, ptr %_51, align 1
; call alloc::string::String::new
  call void @_ZN5alloc6string6String3new17h3044a7d4920107efE(ptr sret(%"alloc::string::String") %line)
; invoke core::fmt::Arguments::new_v1
  invoke void @_ZN4core3fmt9Arguments6new_v117h052a350a7920a495E(ptr sret(%"core::fmt::Arguments<'_>") %_3, ptr align 8 @alloc4, i64 1, ptr align 8 @alloc13, i64 0)
          to label %bb2 unwind label %cleanup

bb20:                                             ; preds = %bb18, %cleanup
  %1 = load i8, ptr %_51, align 1, !range !6, !noundef !3
  %2 = trunc i8 %1 to i1
  br i1 %2, label %bb19, label %bb16

cleanup:                                          ; preds = %bb5, %bb4, %bb3, %bb2, %start
  %3 = landingpad { ptr, i32 }
          cleanup
  %4 = extractvalue { ptr, i32 } %3, 0
  %5 = extractvalue { ptr, i32 } %3, 1
  %6 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0
  store ptr %4, ptr %6, align 8
  %7 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  store i32 %5, ptr %7, align 8
  br label %bb20

bb2:                                              ; preds = %start
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17hb0eaeec85cb65187E(ptr %_3)
          to label %bb3 unwind label %cleanup

bb3:                                              ; preds = %bb2
; invoke std::io::stdio::stdin
  %8 = invoke align 8 ptr @_ZN3std2io5stdio5stdin17hdd1e9d780a25bac9E()
          to label %bb4 unwind label %cleanup

bb4:                                              ; preds = %bb3
  store ptr %8, ptr %_13, align 8
; invoke std::io::stdio::Stdin::read_line
  invoke void @_ZN3std2io5stdio5Stdin9read_line17he9a2c8f04a028466E(ptr sret(%"core::result::Result<usize, std::io::error::Error>") %_11, ptr align 8 %_13, ptr align 8 %line)
          to label %bb5 unwind label %cleanup

bb5:                                              ; preds = %bb4
; invoke core::result::Result<T,E>::unwrap
  %b1 = invoke i64 @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17he320ea76755b89c2E"(ptr %_11, ptr align 8 @alloc139)
          to label %bb6 unwind label %cleanup

bb6:                                              ; preds = %bb5
  store i8 0, ptr %_51, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_17, ptr align 8 %line, i64 24, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %v, ptr align 8 %_17, i64 24, i1 false)
  %9 = getelementptr inbounds %"e<alloc::string::String, i32>", ptr %v, i32 0, i32 1
  %10 = load ptr, ptr %9, align 8
  %11 = ptrtoint ptr %10 to i64
  %12 = icmp eq i64 %11, 0
  %_18 = select i1 %12, i64 1, i64 0
  %13 = icmp eq i64 %_18, 0
  br i1 %13, label %bb9, label %bb7

bb9:                                              ; preds = %bb6
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %v1, ptr align 8 %v, i64 24, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %x, ptr align 8 %v1, i64 24, i1 false)
; invoke core::fmt::ArgumentV1::new_debug
  %14 = invoke { ptr, ptr } @_ZN4core3fmt10ArgumentV19new_debug17ha7236a35225c120eE(ptr align 8 %x)
          to label %bb10 unwind label %cleanup5

bb7:                                              ; preds = %bb6
  %v3 = load i32, ptr %v, align 8
  store i32 %v3, ptr %x2, align 4
; invoke core::fmt::ArgumentV1::new_debug
  %15 = invoke { ptr, ptr } @_ZN4core3fmt10ArgumentV19new_debug17h1373fa05f9187e2bE(ptr align 4 %x2)
          to label %bb13 unwind label %cleanup4

bb8:                                              ; No predecessors!
  unreachable

bb18:                                             ; preds = %bb15, %cleanup4
  br label %bb20

cleanup4:                                         ; preds = %bb12, %bb14, %bb13, %bb7
  %16 = landingpad { ptr, i32 }
          cleanup
  %17 = extractvalue { ptr, i32 } %16, 0
  %18 = extractvalue { ptr, i32 } %16, 1
  %19 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0
  store ptr %17, ptr %19, align 8
  %20 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  store i32 %18, ptr %20, align 8
  br label %bb18

bb13:                                             ; preds = %bb7
  %_44.0 = extractvalue { ptr, ptr } %15, 0
  %_44.1 = extractvalue { ptr, ptr } %15, 1
  %21 = getelementptr inbounds [1 x { ptr, ptr }], ptr %_43, i64 0, i64 0
  %22 = getelementptr inbounds { ptr, ptr }, ptr %21, i32 0, i32 0
  store ptr %_44.0, ptr %22, align 8
  %23 = getelementptr inbounds { ptr, ptr }, ptr %21, i32 0, i32 1
  store ptr %_44.1, ptr %23, align 8
; invoke core::fmt::Arguments::new_v1
  invoke void @_ZN4core3fmt9Arguments6new_v117h052a350a7920a495E(ptr sret(%"core::fmt::Arguments<'_>") %_36, ptr align 8 @alloc14, i64 2, ptr align 8 %_43, i64 1)
          to label %bb14 unwind label %cleanup4

bb14:                                             ; preds = %bb13
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17hb0eaeec85cb65187E(ptr %_36)
          to label %bb21 unwind label %cleanup4

bb21:                                             ; preds = %bb14
  br label %bb17

bb17:                                             ; preds = %bb12, %bb21
  store i8 0, ptr %_51, align 1
  ret void

bb15:                                             ; preds = %cleanup5
; invoke core::ptr::drop_in_place<alloc::string::String>
  invoke void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hb63df47af4d3dd7aE"(ptr %x) #12
          to label %bb18 unwind label %abort

cleanup5:                                         ; preds = %bb11, %bb10, %bb9
  %24 = landingpad { ptr, i32 }
          cleanup
  %25 = extractvalue { ptr, i32 } %24, 0
  %26 = extractvalue { ptr, i32 } %24, 1
  %27 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0
  store ptr %25, ptr %27, align 8
  %28 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  store i32 %26, ptr %28, align 8
  br label %bb15

bb10:                                             ; preds = %bb9
  %_30.0 = extractvalue { ptr, ptr } %14, 0
  %_30.1 = extractvalue { ptr, ptr } %14, 1
  %29 = getelementptr inbounds [1 x { ptr, ptr }], ptr %_29, i64 0, i64 0
  %30 = getelementptr inbounds { ptr, ptr }, ptr %29, i32 0, i32 0
  store ptr %_30.0, ptr %30, align 8
  %31 = getelementptr inbounds { ptr, ptr }, ptr %29, i32 0, i32 1
  store ptr %_30.1, ptr %31, align 8
; invoke core::fmt::Arguments::new_v1
  invoke void @_ZN4core3fmt9Arguments6new_v117h052a350a7920a495E(ptr sret(%"core::fmt::Arguments<'_>") %_22, ptr align 8 @alloc14, i64 2, ptr align 8 %_29, i64 1)
          to label %bb11 unwind label %cleanup5

bb11:                                             ; preds = %bb10
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17hb0eaeec85cb65187E(ptr %_22)
          to label %bb12 unwind label %cleanup5

bb12:                                             ; preds = %bb11
; invoke core::ptr::drop_in_place<alloc::string::String>
  invoke void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hb63df47af4d3dd7aE"(ptr %x)
          to label %bb17 unwind label %cleanup4

abort:                                            ; preds = %bb19, %bb15
  %32 = landingpad { ptr, i32 }
          cleanup
; call core::panicking::panic_no_unwind
  call void @_ZN4core9panicking15panic_no_unwind17hb07180a78460155fE() #13
  unreachable

bb16:                                             ; preds = %bb19, %bb20
  %33 = load ptr, ptr %0, align 8
  %34 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  %35 = load i32, ptr %34, align 8
  %36 = insertvalue { ptr, i32 } undef, ptr %33, 0
  %37 = insertvalue { ptr, i32 } %36, i32 %35, 1
  resume { ptr, i32 } %37

bb19:                                             ; preds = %bb20
; invoke core::ptr::drop_in_place<alloc::string::String>
  invoke void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hb63df47af4d3dd7aE"(ptr %line) #12
          to label %bb16 unwind label %abort
}

; Function Attrs: nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #2

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17h341545107301821dE(ptr align 1, i64, ptr align 8) unnamed_addr #4

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare i64 @_ZN3std2rt19lang_start_internal17h79190e3a877a769dE(ptr align 1, ptr align 8, i64, ptr, i8) unnamed_addr #2

; core::fmt::Formatter::debug_lower_hex
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17hacc814ad62bccd75E(ptr align 8) unnamed_addr #2

; core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h39ae24742179d814E"(ptr align 4, ptr align 8) unnamed_addr #2

; core::fmt::Formatter::debug_upper_hex
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17h1de7f6350f80b832E(ptr align 8) unnamed_addr #2

; core::fmt::num::<impl core::fmt::UpperHex for i32>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17h5f0ef384b379ca73E"(ptr align 4, ptr align 8) unnamed_addr #2

; core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h074201b7268d1f5fE"(ptr align 4, ptr align 8) unnamed_addr #2

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking9panic_fmt17hfd9e949092070b66E(ptr, ptr align 8) unnamed_addr #4

; core::panicking::panic_no_unwind
; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable
declare void @_ZN4core9panicking15panic_no_unwind17hb07180a78460155fE() unnamed_addr #5

; Function Attrs: argmemonly nocallback nofree nounwind willreturn
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #6

; Function Attrs: inaccessiblememonly nocallback nofree nosync nounwind willreturn
declare void @llvm.assume(i1 noundef) #7

; Function Attrs: nocallback nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #8

; <std::io::error::Error as core::fmt::Debug>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN58_$LT$std..io..error..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h5f20ceb8e82723c0E"(ptr align 8, ptr align 8) unnamed_addr #2

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core6result13unwrap_failed17h4d34d8346233eb49E(ptr align 1, i64, ptr align 1, ptr align 8, ptr align 8) unnamed_addr #4

; <str as core::fmt::Debug>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17h4c1c0ac714e96665E"(ptr align 1, i64, ptr align 8) unnamed_addr #2

; Function Attrs: nounwind nonlazybind allockind("free") uwtable
declare void @__rust_dealloc(ptr allocptr, i64, i64) unnamed_addr #9

; std::io::stdio::_print
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std2io5stdio6_print17hb0eaeec85cb65187E(ptr) unnamed_addr #2

; std::io::stdio::stdin
; Function Attrs: nonlazybind uwtable
declare align 8 ptr @_ZN3std2io5stdio5stdin17hdd1e9d780a25bac9E() unnamed_addr #2

; std::io::stdio::Stdin::read_line
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std2io5stdio5Stdin9read_line17he9a2c8f04a028466E(ptr sret(%"core::result::Result<usize, std::io::error::Error>"), ptr align 8, ptr align 8) unnamed_addr #2

; Function Attrs: nonlazybind
define i32 @main(i32 %0, ptr %1) unnamed_addr #10 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17hd4c35bbfd2a30fb8E(ptr @_ZN1f4main17h583fb028a6d62acbE, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { noinline nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #2 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #3 = { inlinehint noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #4 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #5 = { cold noinline noreturn nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #6 = { argmemonly nocallback nofree nounwind willreturn }
attributes #7 = { inaccessiblememonly nocallback nofree nosync nounwind willreturn }
attributes #8 = { nocallback nofree nosync nounwind readnone willreturn }
attributes #9 = { nounwind nonlazybind allockind("free") uwtable "alloc-family"="__rust_alloc" "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #10 = { nonlazybind "target-cpu"="x86-64" }
attributes #11 = { noreturn }
attributes #12 = { noinline }
attributes #13 = { noinline noreturn nounwind }
attributes #14 = { nounwind }

!llvm.module.flags = !{!0, !1, !2}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
!3 = !{}
!4 = !{i32 2317067}
!5 = !{i8 0, i8 42}
!6 = !{i8 0, i8 2}
!7 = !{i64 8}
!8 = !{i8 0, i8 41}
!9 = !{i64 1}
!10 = !{i8 0, i8 4}
!11 = !{i8 0, i8 3}
!12 = !{i64 1, i64 -9223372036854775807}
!13 = !{i64 0, i64 -9223372036854775807}
!14 = !{i64 0, i64 2}
!15 = !{i64 1, i64 0}
