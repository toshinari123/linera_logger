; ModuleID = 'probe7.4a31b90ba17e58b0-cgu.0'
source_filename = "probe7.4a31b90ba17e58b0-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

; std::f64::<impl f64>::copysign
; Function Attrs: inlinehint nounwind
define internal double @"_ZN3std3f6421_$LT$impl$u20$f64$GT$8copysign17h6a74f41a783a7a5aE"(double %self, double %sign) unnamed_addr #0 {
start:
  %0 = alloca double, align 8
  %1 = call double @llvm.copysign.f64(double %self, double %sign)
  store double %1, ptr %0, align 8
  %2 = load double, ptr %0, align 8, !noundef !0
  ret double %2
}

; probe7::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe75probe17hbdcc35bb1cde0155E() unnamed_addr #1 {
start:
; call std::f64::<impl f64>::copysign
  %_1 = call double @"_ZN3std3f6421_$LT$impl$u20$f64$GT$8copysign17h6a74f41a783a7a5aE"(double 1.000000e+00, double -1.000000e+00) #3
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare hidden double @llvm.copysign.f64(double, double) #2

attributes #0 = { inlinehint nounwind "target-cpu"="generic" }
attributes #1 = { nounwind "target-cpu"="generic" }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #3 = { nounwind }

!0 = !{}
