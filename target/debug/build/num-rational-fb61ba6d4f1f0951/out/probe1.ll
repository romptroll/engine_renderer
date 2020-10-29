; ModuleID = 'probe1.3a1fbbbh-cgu.0'
source_filename = "probe1.3a1fbbbh-cgu.0"
target datalayout = "e-m:w-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

%"core::fmt::Formatter" = type { [0 x i64], { i64, i64 }, [0 x i64], { i64, i64 }, [0 x i64], { {}*, [3 x i64]* }, [0 x i32], i32, [0 x i32], i32, [0 x i8], i8, [7 x i8] }
%"core::fmt::::Opaque" = type {}
%"core::fmt::Arguments" = type { [0 x i64], { [0 x { [0 x i8]*, i64 }]*, i64 }, [0 x i64], { i64*, i64 }, [0 x i64], { [0 x { i8*, i8* }]*, i64 }, [0 x i64] }
%"alloc::string::String" = type { [0 x i64], %"alloc::vec::Vec<u8>", [0 x i64] }
%"alloc::vec::Vec<u8>" = type { [0 x i64], { i8*, i64 }, [0 x i64], i64, [0 x i64] }
%"core::ptr::Repr<u8>" = type { [2 x i64] }
%"alloc::alloc::Global" = type {}

@0 = private unnamed_addr constant <{ [0 x i8] }> zeroinitializer, align 1
@1 = private unnamed_addr constant <{ i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [0 x i8] }>, <{ [0 x i8] }>* @0, i32 0, i32 0, i32 0), [8 x i8] zeroinitializer }>, align 8
@2 = private unnamed_addr constant <{ i8*, [0 x i8] }> <{ i8* bitcast (<{ i8*, [8 x i8] }>* @1 to i8*), [0 x i8] zeroinitializer }>, align 8
@3 = private unnamed_addr constant <{ [8 x i8] }> zeroinitializer, align 8
@4 = private unnamed_addr constant <{ i8*, [0 x i8] }> <{ i8* getelementptr inbounds (<{ [8 x i8] }>, <{ [8 x i8] }>* @3, i32 0, i32 0, i32 0), [0 x i8] zeroinitializer }>, align 8

; <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h84fb22dafc88ebb7E"(i8* nonnull %unique) unnamed_addr #0 {
start:
; call core::ptr::unique::Unique<T>::as_ptr
  %_2 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h7f67001ad6eb80ddE"(i8* nonnull %unique)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %0 = call nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h7c26efb8ba93f3d6E"(i8* %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; core::fmt::ArgumentV1::new
; Function Attrs: uwtable
define { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17h405f584d499fe572E(i64* noalias readonly align 8 dereferenceable(8) %x, i1 (i64*, %"core::fmt::Formatter"*)* nonnull %f) unnamed_addr #1 {
start:
  %0 = alloca %"core::fmt::::Opaque"*, align 8
  %1 = alloca i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)*, align 8
  %2 = alloca { i8*, i8* }, align 8
  %3 = bitcast i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)** %1 to i1 (i64*, %"core::fmt::Formatter"*)**
  store i1 (i64*, %"core::fmt::Formatter"*)* %f, i1 (i64*, %"core::fmt::Formatter"*)** %3, align 8
  %_3 = load i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)*, i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)** %1, align 8, !nonnull !1
  br label %bb1

bb1:                                              ; preds = %start
  %4 = bitcast %"core::fmt::::Opaque"** %0 to i64**
  store i64* %x, i64** %4, align 8
  %_5 = load %"core::fmt::::Opaque"*, %"core::fmt::::Opaque"** %0, align 8, !nonnull !1
  br label %bb2

bb2:                                              ; preds = %bb1
  %5 = bitcast { i8*, i8* }* %2 to %"core::fmt::::Opaque"**
  store %"core::fmt::::Opaque"* %_5, %"core::fmt::::Opaque"** %5, align 8
  %6 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 1
  %7 = bitcast i8** %6 to i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)**
  store i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)* %_3, i1 (%"core::fmt::::Opaque"*, %"core::fmt::Formatter"*)** %7, align 8
  %8 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 0
  %9 = load i8*, i8** %8, align 8, !nonnull !1
  %10 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %2, i32 0, i32 1
  %11 = load i8*, i8** %10, align 8, !nonnull !1
  %12 = insertvalue { i8*, i8* } undef, i8* %9, 0
  %13 = insertvalue { i8*, i8* } %12, i8* %11, 1
  ret { i8*, i8* } %13
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117h0a13d6e3b6cbd722E(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(48), [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8 %pieces.0, i64 %pieces.1, [0 x { i8*, i8* }]* noalias nonnull readonly align 8 %args.0, i64 %args.1) unnamed_addr #0 {
start:
  %_4 = alloca { i64*, i64 }, align 8
  %1 = bitcast { i64*, i64 }* %_4 to {}**
  store {}* null, {}** %1, align 8
  %2 = bitcast %"core::fmt::Arguments"* %0 to { [0 x { [0 x i8]*, i64 }]*, i64 }*
  %3 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %2, i32 0, i32 0
  store [0 x { [0 x i8]*, i64 }]* %pieces.0, [0 x { [0 x i8]*, i64 }]** %3, align 8
  %4 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %2, i32 0, i32 1
  store i64 %pieces.1, i64* %4, align 8
  %5 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 3
  %6 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_4, i32 0, i32 0
  %7 = load i64*, i64** %6, align 8
  %8 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_4, i32 0, i32 1
  %9 = load i64, i64* %8, align 8
  %10 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %5, i32 0, i32 0
  store i64* %7, i64** %10, align 8
  %11 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %5, i32 0, i32 1
  store i64 %9, i64* %11, align 8
  %12 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 5
  %13 = getelementptr inbounds { [0 x { i8*, i8* }]*, i64 }, { [0 x { i8*, i8* }]*, i64 }* %12, i32 0, i32 0
  store [0 x { i8*, i8* }]* %args.0, [0 x { i8*, i8* }]** %13, align 8
  %14 = getelementptr inbounds { [0 x { i8*, i8* }]*, i64 }, { [0 x { i8*, i8* }]*, i64 }* %12, i32 0, i32 1
  store i64 %args.1, i64* %14, align 8
  ret void
}

; core::num::NonZeroUsize::new_unchecked
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3num12NonZeroUsize13new_unchecked17h802945f02b87fc0dE(i64 %n) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  store i64 %n, i64* %0, align 8
  %1 = load i64, i64* %0, align 8, !range !2
  ret i64 %1
}

; core::num::NonZeroUsize::get
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3num12NonZeroUsize3get17he9e3639114cf1a2eE(i64 %self) unnamed_addr #0 {
start:
  ret i64 %self
}

; core::ptr::drop_in_place
; Function Attrs: uwtable
define void @_ZN4core3ptr13drop_in_place17h15946d58b87a2efbE({ i8*, i64 }* %_1) unnamed_addr #1 {
start:
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hefa7b4082f0ad576E"({ i8*, i64 }* align 8 dereferenceable(16) %_1)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place
; Function Attrs: uwtable
define void @_ZN4core3ptr13drop_in_place17hc5f0c3fcc0e6012fE(%"alloc::string::String"* %_1) unnamed_addr #1 {
start:
  %0 = bitcast %"alloc::string::String"* %_1 to %"alloc::vec::Vec<u8>"*
; call core::ptr::drop_in_place
  call void @_ZN4core3ptr13drop_in_place17hf50e251c17e0fbc4E(%"alloc::vec::Vec<u8>"* %0)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place
; Function Attrs: uwtable
define void @_ZN4core3ptr13drop_in_place17hf50e251c17e0fbc4E(%"alloc::vec::Vec<u8>"* %_1) unnamed_addr #1 personality i32 (...)* @__CxxFrameHandler3 {
start:
; invoke <alloc::vec::Vec<T> as core::ops::drop::Drop>::drop
  invoke void @"_ZN66_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h93f60a0255cf1979E"(%"alloc::vec::Vec<u8>"* align 8 dereferenceable(24) %_1)
          to label %bb4 unwind label %funclet_bb3

bb1:                                              ; preds = %bb3
  cleanupret from %cleanuppad unwind to caller

bb2:                                              ; preds = %bb4
  ret void

bb3:                                              ; preds = %funclet_bb3
  %0 = bitcast %"alloc::vec::Vec<u8>"* %_1 to { i8*, i64 }*
; call core::ptr::drop_in_place
  call void @_ZN4core3ptr13drop_in_place17h15946d58b87a2efbE({ i8*, i64 }* %0) #6 [ "funclet"(token %cleanuppad) ]
  br label %bb1

bb4:                                              ; preds = %start
  %1 = bitcast %"alloc::vec::Vec<u8>"* %_1 to { i8*, i64 }*
; call core::ptr::drop_in_place
  call void @_ZN4core3ptr13drop_in_place17h15946d58b87a2efbE({ i8*, i64 }* %1)
  br label %bb2

funclet_bb3:                                      ; preds = %start
  %cleanuppad = cleanuppad within none []
  br label %bb3
}

; core::ptr::slice_from_raw_parts_mut
; Function Attrs: inlinehint uwtable
define { [0 x i8]*, i64 } @_ZN4core3ptr24slice_from_raw_parts_mut17h816e3458cc1b890fE(i8* %data, i64 %len) unnamed_addr #0 {
start:
  %_4 = alloca { i8*, i64 }, align 8
  %_3 = alloca %"core::ptr::Repr<u8>", align 8
  %0 = bitcast { i8*, i64 }* %_4 to i8**
  store i8* %data, i8** %0, align 8
  %1 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 1
  store i64 %len, i64* %1, align 8
  %2 = bitcast %"core::ptr::Repr<u8>"* %_3 to { i8*, i64 }*
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 0
  %4 = load i8*, i8** %3, align 8
  %5 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_4, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 0
  store i8* %4, i8** %7, align 8
  %8 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %2, i32 0, i32 1
  store i64 %6, i64* %8, align 8
  %9 = bitcast %"core::ptr::Repr<u8>"* %_3 to { [0 x i8]*, i64 }*
  %10 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %9, i32 0, i32 0
  %11 = load [0 x i8]*, [0 x i8]** %10, align 8
  %12 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %9, i32 0, i32 1
  %13 = load i64, i64* %12, align 8
  %14 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %11, 0
  %15 = insertvalue { [0 x i8]*, i64 } %14, i64 %13, 1
  ret { [0 x i8]*, i64 } %15
}

; core::ptr::unique::Unique<T>::as_ptr
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h7f67001ad6eb80ddE"(i8* nonnull %self) unnamed_addr #0 {
start:
  ret i8* %self
}

; core::ptr::mut_ptr::<impl *mut T>::is_null
; Function Attrs: inlinehint uwtable
define zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hfaba11be5ad4a3b4E"(i8* %self) unnamed_addr #0 {
start:
  br label %bb1

bb1:                                              ; preds = %start
  %0 = icmp eq i8* %self, null
  ret i1 %0
}

; core::ptr::non_null::NonNull<T>::new_unchecked
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h7c26efb8ba93f3d6E"(i8* %ptr) unnamed_addr #0 {
start:
  %0 = alloca i8*, align 8
  store i8* %ptr, i8** %0, align 8
  %1 = load i8*, i8** %0, align 8, !nonnull !1
  ret i8* %1
}

; core::ptr::non_null::NonNull<T>::cast
; Function Attrs: inlinehint uwtable
define nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$4cast17h06a666fe4b840b6dE"(i8* nonnull %self) unnamed_addr #0 {
start:
; call core::ptr::non_null::NonNull<T>::as_ptr
  %_3 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hf5b3610be171bc20E"(i8* nonnull %self)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::non_null::NonNull<T>::new_unchecked
  %0 = call nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h7c26efb8ba93f3d6E"(i8* %_3)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i8* %0
}

; core::ptr::non_null::NonNull<T>::as_ptr
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hf5b3610be171bc20E"(i8* nonnull %self) unnamed_addr #0 {
start:
  ret i8* %self
}

; core::alloc::Layout::from_size_align_unchecked
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @_ZN4core5alloc6Layout25from_size_align_unchecked17h5a55b70c441560a3E(i64 %size, i64 %align) unnamed_addr #0 {
start:
  %0 = alloca { i64, i64 }, align 8
; call core::num::NonZeroUsize::new_unchecked
  %_4 = call i64 @_ZN4core3num12NonZeroUsize13new_unchecked17h802945f02b87fc0dE(i64 %align), !range !2
  br label %bb1

bb1:                                              ; preds = %start
  %1 = bitcast { i64, i64 }* %0 to i64*
  store i64 %size, i64* %1, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  store i64 %_4, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 0
  %4 = load i64, i64* %3, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %0, i32 0, i32 1
  %6 = load i64, i64* %5, align 8, !range !2
  %7 = insertvalue { i64, i64 } undef, i64 %4, 0
  %8 = insertvalue { i64, i64 } %7, i64 %6, 1
  ret { i64, i64 } %8
}

; core::alloc::Layout::size
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core5alloc6Layout4size17he5b5338797143501E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = bitcast { i64, i64 }* %self to i64*
  %1 = load i64, i64* %0, align 8
  ret i64 %1
}

; core::alloc::Layout::align
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core5alloc6Layout5align17hce94ba71c750d0deE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  %_2 = load i64, i64* %0, align 8, !range !2
; call core::num::NonZeroUsize::get
  %1 = call i64 @_ZN4core3num12NonZeroUsize3get17he9e3639114cf1a2eE(i64 %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; core::slice::from_raw_parts_mut
; Function Attrs: inlinehint uwtable
define { [0 x i8]*, i64 } @_ZN4core5slice18from_raw_parts_mut17h5db500135f0ffaabE(i8* %data, i64 %len) unnamed_addr #0 {
start:
; call core::ptr::slice_from_raw_parts_mut
  %0 = call { [0 x i8]*, i64 } @_ZN4core3ptr24slice_from_raw_parts_mut17h816e3458cc1b890fE(i8* %data, i64 %len)
  %_5.0 = extractvalue { [0 x i8]*, i64 } %0, 0
  %_5.1 = extractvalue { [0 x i8]*, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %1 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %_5.0, 0
  %2 = insertvalue { [0 x i8]*, i64 } %1, i64 %_5.1, 1
  ret { [0 x i8]*, i64 } %2
}

; core::slice::<impl core::ops::index::IndexMut<I> for [T]>::index_mut
; Function Attrs: inlinehint uwtable
define { [0 x i8]*, i64 } @"_ZN4core5slice77_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$9index_mut17h1b4272aeeaaf8785E"([0 x i8]* nonnull align 1 %self.0, i64 %self.1) unnamed_addr #0 {
start:
; call <core::ops::range::RangeFull as core::slice::SliceIndex<[T]>>::index_mut
  %0 = call { [0 x i8]*, i64 } @"_ZN90_$LT$core..ops..range..RangeFull$u20$as$u20$core..slice..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$9index_mut17he8bb1fb0c43aaa07E"([0 x i8]* nonnull align 1 %self.0, i64 %self.1)
  %_4.0 = extractvalue { [0 x i8]*, i64 } %0, 0
  %_4.1 = extractvalue { [0 x i8]*, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %1 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %_4.0, 0
  %2 = insertvalue { [0 x i8]*, i64 } %1, i64 %_4.1, 1
  ret { [0 x i8]*, i64 } %2
}

; alloc::vec::Vec<T>::as_mut_ptr
; Function Attrs: inlinehint uwtable
define i8* @"_ZN5alloc3vec12Vec$LT$T$GT$10as_mut_ptr17he3d21ea80e1b0352E"(%"alloc::vec::Vec<u8>"* align 8 dereferenceable(24) %self) unnamed_addr #0 {
start:
  %_3 = bitcast %"alloc::vec::Vec<u8>"* %self to { i8*, i64 }*
; call alloc::raw_vec::RawVec<T,A>::ptr
  %ptr = call i8* @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17h800d601f735e9f48E"({ i8*, i64 }* noalias readonly align 8 dereferenceable(16) %_3)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::mut_ptr::<impl *mut T>::is_null
  %_6 = call zeroext i1 @"_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hfaba11be5ad4a3b4E"(i8* %ptr)
  br label %bb2

bb2:                                              ; preds = %bb1
  %_5 = xor i1 %_6, true
  call void @llvm.assume(i1 %_5)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i8* %ptr
}

; alloc::alloc::dealloc
; Function Attrs: inlinehint uwtable
define internal void @_ZN5alloc5alloc7dealloc17h6aaba6abe9dd7d69E(i8* %ptr, i64, i64) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::Layout::size
  %_4 = call i64 @_ZN4core5alloc6Layout4size17he5b5338797143501E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
; call core::alloc::Layout::align
  %_6 = call i64 @_ZN4core5alloc6Layout5align17hce94ba71c750d0deE({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb2

bb2:                                              ; preds = %bb1
  call void @__rust_dealloc(i8* %ptr, i64 %_4, i64 %_6)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret void
}

; alloc::raw_vec::RawVec<T,A>::current_layout
; Function Attrs: uwtable
define { i64, i64 } @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_layout17h69a24bf56c89c857E"({ i8*, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
  %0 = alloca i64, align 8
  %1 = alloca i64, align 8
  %2 = alloca { i64, i64 }, align 8
  %3 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i32 0, i32 1
  %_3 = load i64, i64* %3, align 8
  %_2 = icmp eq i64 %_3, 0
  br i1 %_2, label %bb2, label %bb1

bb1:                                              ; preds = %start
  store i64 1, i64* %1, align 8
  %4 = load i64, i64* %1, align 8
  br label %bb3

bb2:                                              ; preds = %start
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %2, i32 0, i32 1
  store i64 0, i64* %5, align 8
  br label %bb6

bb3:                                              ; preds = %bb1
  store i64 1, i64* %0, align 8
  %6 = load i64, i64* %0, align 8
  br label %bb4

bb4:                                              ; preds = %bb3
  %7 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i32 0, i32 1
  %_7 = load i64, i64* %7, align 8
  %size = mul i64 %6, %_7
; call core::alloc::Layout::from_size_align_unchecked
  %8 = call { i64, i64 } @_ZN4core5alloc6Layout25from_size_align_unchecked17h5a55b70c441560a3E(i64 %size, i64 %4)
  %_8.0 = extractvalue { i64, i64 } %8, 0
  %_8.1 = extractvalue { i64, i64 } %8, 1
  br label %bb5

bb5:                                              ; preds = %bb4
  %9 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %2, i32 0, i32 0
  store i64 %_8.0, i64* %9, align 8
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %2, i32 0, i32 1
  store i64 %_8.1, i64* %10, align 8
  br label %bb6

bb6:                                              ; preds = %bb5, %bb2
  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %2, i32 0, i32 0
  %12 = load i64, i64* %11, align 8
  %13 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %2, i32 0, i32 1
  %14 = load i64, i64* %13, align 8
  %15 = insertvalue { i64, i64 } undef, i64 %12, 0
  %16 = insertvalue { i64, i64 } %15, i64 %14, 1
  ret { i64, i64 } %16
}

; alloc::raw_vec::RawVec<T,A>::dealloc_buffer
; Function Attrs: uwtable
define void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14dealloc_buffer17h42010d81214f2a18E"({ i8*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
  %0 = alloca i64, align 8
  %_5 = alloca { i64, i64 }, align 8
  store i64 1, i64* %0, align 8
  %1 = load i64, i64* %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  %_3 = icmp ne i64 %1, 0
  br i1 %_3, label %bb2, label %bb9

bb2:                                              ; preds = %bb1
; call alloc::raw_vec::RawVec<T,A>::current_layout
  %2 = call { i64, i64 } @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_layout17h69a24bf56c89c857E"({ i8*, i64 }* noalias readonly align 8 dereferenceable(16) %self)
  store { i64, i64 } %2, { i64, i64 }* %_5, align 8
  br label %bb3

bb3:                                              ; preds = %bb2
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_5, i32 0, i32 1
  %4 = load i64, i64* %3, align 8
  %5 = icmp ule i64 %4, 0
  %_7 = select i1 %5, i64 0, i64 1
  %6 = icmp eq i64 %_7, 1
  br i1 %6, label %bb4, label %bb8

bb4:                                              ; preds = %bb3
  %7 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_5, i32 0, i32 0
  %layout.0 = load i64, i64* %7, align 8
  %8 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_5, i32 0, i32 1
  %layout.1 = load i64, i64* %8, align 8, !range !2
  %_10 = bitcast { i8*, i64 }* %self to %"alloc::alloc::Global"*
  %9 = bitcast { i8*, i64 }* %self to i8**
  %_13 = load i8*, i8** %9, align 8, !nonnull !1
; call <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
  %_12 = call nonnull i8* @"_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h84fb22dafc88ebb7E"(i8* nonnull %_13)
  br label %bb5

bb5:                                              ; preds = %bb4
; call core::ptr::non_null::NonNull<T>::cast
  %_11 = call nonnull i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$4cast17h06a666fe4b840b6dE"(i8* nonnull %_12)
  br label %bb6

bb6:                                              ; preds = %bb5
; call <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
  call void @"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17h921b853bda60f27dE"(%"alloc::alloc::Global"* nonnull align 1 %_10, i8* nonnull %_11, i64 %layout.0, i64 %layout.1)
  br label %bb7

bb7:                                              ; preds = %bb6
  br label %bb8

bb8:                                              ; preds = %bb7, %bb3
  br label %bb9

bb9:                                              ; preds = %bb8, %bb1
  ret void
}

; alloc::raw_vec::RawVec<T,A>::ptr
; Function Attrs: uwtable
define i8* @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17h800d601f735e9f48E"({ i8*, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
  %0 = bitcast { i8*, i64 }* %self to i8**
  %_2 = load i8*, i8** %0, align 8, !nonnull !1
; call core::ptr::unique::Unique<T>::as_ptr
  %1 = call i8* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h7f67001ad6eb80ddE"(i8* nonnull %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %1
}

; <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
; Function Attrs: inlinehint uwtable
define internal void @"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17h921b853bda60f27dE"(%"alloc::alloc::Global"* nonnull align 1 %self, i8* nonnull %ptr, i64, i64) unnamed_addr #0 {
start:
  %layout = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  store i64 %1, i64* %3, align 8
; call core::alloc::Layout::size
  %_5 = call i64 @_ZN4core5alloc6Layout4size17he5b5338797143501E({ i64, i64 }* noalias readonly align 8 dereferenceable(16) %layout)
  br label %bb1

bb1:                                              ; preds = %start
  %_4 = icmp ne i64 %_5, 0
  br i1 %_4, label %bb2, label %bb5

bb2:                                              ; preds = %bb1
; call core::ptr::non_null::NonNull<T>::as_ptr
  %_7 = call i8* @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hf5b3610be171bc20E"(i8* nonnull %ptr)
  br label %bb3

bb3:                                              ; preds = %bb2
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0
  %_9.0 = load i64, i64* %4, align 8
  %5 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1
  %_9.1 = load i64, i64* %5, align 8, !range !2
; call alloc::alloc::dealloc
  call void @_ZN5alloc5alloc7dealloc17h6aaba6abe9dd7d69E(i8* %_7, i64 %_9.0, i64 %_9.1)
  br label %bb4

bb4:                                              ; preds = %bb3
  br label %bb5

bb5:                                              ; preds = %bb4, %bb1
  ret void
}

; <alloc::vec::Vec<T> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define void @"_ZN66_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h93f60a0255cf1979E"(%"alloc::vec::Vec<u8>"* align 8 dereferenceable(24) %self) unnamed_addr #1 {
start:
; call <alloc::vec::Vec<T> as core::ops::index::IndexMut<I>>::index_mut
  %0 = call { [0 x i8]*, i64 } @"_ZN80_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..index..IndexMut$LT$I$GT$$GT$9index_mut17he4187c973c8e8f54E"(%"alloc::vec::Vec<u8>"* align 8 dereferenceable(24) %self)
  %_5.0 = extractvalue { [0 x i8]*, i64 } %0, 0
  %_5.1 = extractvalue { [0 x i8]*, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; <alloc::vec::Vec<T> as core::ops::deref::DerefMut>::deref_mut
; Function Attrs: uwtable
define { [0 x i8]*, i64 } @"_ZN71_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..deref..DerefMut$GT$9deref_mut17h6281c65789566aacE"(%"alloc::vec::Vec<u8>"* align 8 dereferenceable(24) %self) unnamed_addr #1 {
start:
; call alloc::vec::Vec<T>::as_mut_ptr
  %_5 = call i8* @"_ZN5alloc3vec12Vec$LT$T$GT$10as_mut_ptr17he3d21ea80e1b0352E"(%"alloc::vec::Vec<u8>"* align 8 dereferenceable(24) %self)
  br label %bb1

bb1:                                              ; preds = %start
  %0 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %self, i32 0, i32 3
  %_7 = load i64, i64* %0, align 8
; call core::slice::from_raw_parts_mut
  %1 = call { [0 x i8]*, i64 } @_ZN4core5slice18from_raw_parts_mut17h5db500135f0ffaabE(i8* %_5, i64 %_7)
  %_4.0 = extractvalue { [0 x i8]*, i64 } %1, 0
  %_4.1 = extractvalue { [0 x i8]*, i64 } %1, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %2 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %_4.0, 0
  %3 = insertvalue { [0 x i8]*, i64 } %2, i64 %_4.1, 1
  ret { [0 x i8]*, i64 } %3
}

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hefa7b4082f0ad576E"({ i8*, i64 }* align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
; call alloc::raw_vec::RawVec<T,A>::dealloc_buffer
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14dealloc_buffer17h42010d81214f2a18E"({ i8*, i64 }* align 8 dereferenceable(16) %self)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; <alloc::vec::Vec<T> as core::ops::index::IndexMut<I>>::index_mut
; Function Attrs: inlinehint uwtable
define { [0 x i8]*, i64 } @"_ZN80_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..index..IndexMut$LT$I$GT$$GT$9index_mut17he4187c973c8e8f54E"(%"alloc::vec::Vec<u8>"* align 8 dereferenceable(24) %self) unnamed_addr #0 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %_10 = alloca i8, align 1
  store i8 0, i8* %_10, align 1
  store i8 1, i8* %_10, align 1
; invoke <alloc::vec::Vec<T> as core::ops::deref::DerefMut>::deref_mut
  %0 = invoke { [0 x i8]*, i64 } @"_ZN71_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..deref..DerefMut$GT$9deref_mut17h6281c65789566aacE"(%"alloc::vec::Vec<u8>"* align 8 dereferenceable(24) %self)
          to label %bb2 unwind label %funclet_bb5

bb1:                                              ; preds = %bb4, %bb5
  cleanupret from %cleanuppad unwind to caller

bb2:                                              ; preds = %start
  %_7.0 = extractvalue { [0 x i8]*, i64 } %0, 0
  %_7.1 = extractvalue { [0 x i8]*, i64 } %0, 1
  store i8 0, i8* %_10, align 1
; invoke core::slice::<impl core::ops::index::IndexMut<I> for [T]>::index_mut
  %1 = invoke { [0 x i8]*, i64 } @"_ZN4core5slice77_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$9index_mut17h1b4272aeeaaf8785E"([0 x i8]* nonnull align 1 %_7.0, i64 %_7.1)
          to label %bb3 unwind label %funclet_bb5

bb3:                                              ; preds = %bb2
  %_4.0 = extractvalue { [0 x i8]*, i64 } %1, 0
  %_4.1 = extractvalue { [0 x i8]*, i64 } %1, 1
  %2 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %_4.0, 0
  %3 = insertvalue { [0 x i8]*, i64 } %2, i64 %_4.1, 1
  ret { [0 x i8]*, i64 } %3

bb4:                                              ; preds = %bb5
  store i8 0, i8* %_10, align 1
  br label %bb1

bb5:                                              ; preds = %funclet_bb5
  %4 = load i8, i8* %_10, align 1, !range !3
  %5 = trunc i8 %4 to i1
  br i1 %5, label %bb4, label %bb1

funclet_bb5:                                      ; preds = %bb2, %start
  %cleanuppad = cleanuppad within none []
  br label %bb5
}

; <core::ops::range::RangeFull as core::slice::SliceIndex<[T]>>::index_mut
; Function Attrs: inlinehint uwtable
define { [0 x i8]*, i64 } @"_ZN90_$LT$core..ops..range..RangeFull$u20$as$u20$core..slice..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$9index_mut17he8bb1fb0c43aaa07E"([0 x i8]* nonnull align 1 %slice.0, i64 %slice.1) unnamed_addr #0 {
start:
  %0 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %slice.0, 0
  %1 = insertvalue { [0 x i8]*, i64 } %0, i64 %slice.1, 1
  ret { [0 x i8]*, i64 } %1
}

; probe1::probe
; Function Attrs: uwtable
define void @_ZN6probe15probe17he203a3b61d9abeacE() unnamed_addr #1 {
start:
  %_11 = alloca i64*, align 8
  %_10 = alloca [1 x { i8*, i8* }], align 8
  %_3 = alloca %"core::fmt::Arguments", align 8
  %res = alloca %"alloc::string::String", align 8
  %_1 = alloca %"alloc::string::String", align 8
  %_18 = load [1 x { [0 x i8]*, i64 }]*, [1 x { [0 x i8]*, i64 }]** bitcast (<{ i8*, [0 x i8] }>* @2 to [1 x { [0 x i8]*, i64 }]**), align 8, !nonnull !1
  %_4.0 = bitcast [1 x { [0 x i8]*, i64 }]* %_18 to [0 x { [0 x i8]*, i64 }]*
  %_17 = load i64*, i64** bitcast (<{ i8*, [0 x i8] }>* @4 to i64**), align 8, !nonnull !1
  store i64* %_17, i64** %_11, align 8
  %arg0 = load i64*, i64** %_11, align 8, !nonnull !1
; call core::fmt::ArgumentV1::new
  %0 = call { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17h405f584d499fe572E(i64* noalias readonly align 8 dereferenceable(8) %arg0, i1 (i64*, %"core::fmt::Formatter"*)* nonnull @"_ZN4core3fmt3num3imp55_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$isize$GT$3fmt17ha4f73c009475a4ecE")
  %_14.0 = extractvalue { i8*, i8* } %0, 0
  %_14.1 = extractvalue { i8*, i8* } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
  %1 = bitcast [1 x { i8*, i8* }]* %_10 to { i8*, i8* }*
  %2 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %1, i32 0, i32 0
  store i8* %_14.0, i8** %2, align 8
  %3 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %1, i32 0, i32 1
  store i8* %_14.1, i8** %3, align 8
  %_7.0 = bitcast [1 x { i8*, i8* }]* %_10 to [0 x { i8*, i8* }]*
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h0a13d6e3b6cbd722E(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %_3, [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8 %_4.0, i64 1, [0 x { i8*, i8* }]* noalias nonnull readonly align 8 %_7.0, i64 1)
  br label %bb2

bb2:                                              ; preds = %bb1
; call alloc::fmt::format
  call void @_ZN5alloc3fmt6format17h8a7f457e8f349b77E(%"alloc::string::String"* noalias nocapture sret dereferenceable(24) %res, %"core::fmt::Arguments"* noalias nocapture dereferenceable(48) %_3)
  br label %bb3

bb3:                                              ; preds = %bb2
  %4 = bitcast %"alloc::string::String"* %_1 to i8*
  %5 = bitcast %"alloc::string::String"* %res to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %4, i8* align 8 %5, i64 24, i1 false)
; call core::ptr::drop_in_place
  call void @_ZN4core3ptr13drop_in_place17hc5f0c3fcc0e6012fE(%"alloc::string::String"* %_1)
  br label %bb4

bb4:                                              ; preds = %bb3
  ret void
}

declare i32 @__CxxFrameHandler3(...) unnamed_addr #2

; Function Attrs: nounwind
declare void @llvm.assume(i1) #3

; Function Attrs: nounwind uwtable
declare void @__rust_dealloc(i8*, i64, i64) unnamed_addr #4

; core::fmt::num::imp::<impl core::fmt::LowerExp for isize>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp55_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$isize$GT$3fmt17ha4f73c009475a4ecE"(i64* noalias readonly align 8 dereferenceable(8), %"core::fmt::Formatter"* align 8 dereferenceable(64)) unnamed_addr #1

; alloc::fmt::format
; Function Attrs: uwtable
declare void @_ZN5alloc3fmt6format17h8a7f457e8f349b77E(%"alloc::string::String"* noalias nocapture sret dereferenceable(24), %"core::fmt::Arguments"* noalias nocapture dereferenceable(48)) unnamed_addr #1

; Function Attrs: argmemonly nounwind
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture writeonly, i8* nocapture readonly, i64, i1 immarg) #5

attributes #0 = { inlinehint uwtable "target-cpu"="x86-64" }
attributes #1 = { uwtable "target-cpu"="x86-64" }
attributes #2 = { "target-cpu"="x86-64" }
attributes #3 = { nounwind }
attributes #4 = { nounwind uwtable "target-cpu"="x86-64" }
attributes #5 = { argmemonly nounwind }
attributes #6 = { noinline }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{}
!2 = !{i64 1, i64 0}
!3 = !{i8 0, i8 2}
