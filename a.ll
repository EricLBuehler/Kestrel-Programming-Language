; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone
define void @_main() local_unnamed_addr #0 !dbg !4 {
entry:
  %x = alloca float, !dbg !8
  store float 0x3FF3AE1480000000, float* %x, !dbg !8
  %x1 = load float, float* %x, !dbg !8
  %res = call float @f(float %x1), !dbg !8
  %String = alloca { [12 x i8] }, !dbg !8
  %arr = getelementptr inbounds { [12 x i8] }, { [12 x i8] }* %String, i32 0, i32 0, !dbg !8
  store [12 x i8] c"Kestrel \F0\9F\A6\85", [12 x i8]* %arr, !dbg !8
  %string = load { [12 x i8] }, { [12 x i8] }* %String, !dbg !8
  %name = alloca { [12 x i8] }, !dbg !8
  store { [12 x i8] } %string, { [12 x i8] }* %name, !dbg !8
  %char = alloca i32, !dbg !8
  store i32 129413, i32* %char, !dbg !8
  %x2 = load float, float* %x, !dbg !8
  %x3 = load float, float* %x, !dbg !8
  %f32mul = fmul float %x2, %x3, !dbg !8
  %ftoi = fptosi float %f32mul to i32, !dbg !8
  %arr4 = alloca [2 x i32], !dbg !8
  %i32 = getelementptr [2 x i32], [2 x i32]* %arr4, i8 0, i8 0, !dbg !8
  store i32 %ftoi, i32* %i32, !dbg !8
  %i325 = getelementptr [2 x i32], [2 x i32]* %arr4, i8 0, i8 0, !dbg !8
  store i32 2, i32* %i325, !dbg !8
  %arr6 = alloca [2 x i32]*, !dbg !8
  store [2 x i32]* %arr4, [2 x i32]** %arr6, !dbg !8
  %arr7 = getelementptr inbounds { [12 x i8] }, { [12 x i8] }* %name, i32 0, i32 0, !dbg !8
  %arr8 = load [12 x i8], [12 x i8]* %arr7, !dbg !8
  %len = alloca i64, !dbg !8
  store i64 12, i64* %len, !dbg !8
  %uninit = alloca i64, !dbg !8
  br label %if, !dbg !8

if:                                               ; preds = %entry
  %x9 = alloca i32, !dbg !8
  store i32 100, i32* %x9, !dbg !8
  store i64 1, i64* %uninit, !dbg !8
  br label %if_end, !dbg !8

if_end:                                           ; preds = %if
  br label %loop_head, !dbg !8

loop_head:                                        ; preds = %loop_then, %if_end
  %len11 = load i64, i64* %len, !dbg !8
  %u64lt = icmp uge i64 %len11, %len11, !dbg !8
  %bool = icmp ne i1 %u64lt, false, !dbg !8
  br i1 %bool, label %loop_then, label %loop_end, !dbg !8

loop_then:                                        ; preds = %loop_head
  %len12 = load i64, i64* %len, !dbg !8
  %u64sum = add i64 %len12, 1, !dbg !8
  store i64 %u64sum, i64* %len, !dbg !8
  br label %loop_head, !dbg !8

loop_end:                                         ; preds = %loop_head
  ret void, !dbg !8
}

; Function Attrs: noinline nounwind optnone
define float @f(float %0) local_unnamed_addr #0 !dbg !10 {
entry:
  %x = alloca float, !dbg !14
  store float %0, float* %x, !dbg !14
  %x1 = load float, float* %x, !dbg !14
  %f32sum = fadd float %x1, 1.000000e+00, !dbg !14
  %x2 = load float, float* %x, !dbg !14
  %s = alloca { float, float }, !dbg !14
  %a = getelementptr inbounds { float, float }, { float, float }* %s, i32 0, i32 0, !dbg !14
  store float %f32sum, float* %a, !dbg !14
  %b = getelementptr inbounds { float, float }, { float, float }* %s, i32 0, i32 1, !dbg !14
  store float %x2, float* %b, !dbg !14
  %s3 = load { float, float }, { float, float }* %s, !dbg !14
  %y = alloca { float, float }, !dbg !14
  store { float, float } %s3, { float, float }* %y, !dbg !14
  %s4 = getelementptr inbounds { float, float }, { float, float }* %y, i32 0, i32 0, !dbg !14
  store float 1.200000e+01, float* %s4, !dbg !14
  %s5 = load { float, float }, { float, float }* %y, !dbg !14
  %res = call float @s.a({ float, float } %s5), !dbg !14
  ret float %res, !dbg !14
}

; Function Attrs: noinline nounwind optnone
define float @s.a({ float, float } %0) local_unnamed_addr #0 !dbg !16 {
entry:
  %self = alloca { float, float }, !dbg !20
  store { float, float } %0, { float, float }* %self, !dbg !20
  %s = getelementptr inbounds { float, float }, { float, float }* %self, i32 0, i32 0, !dbg !20
  %a = load float, float* %s, !dbg !20
  %f32mul = fmul float %a, 1.000000e+02, !dbg !20
  ret float %f32mul, !dbg !20
}

; Function Attrs: noinline nounwind optnone
define float @s.add({ float, float } %0, { float, float } %1) local_unnamed_addr #0 !dbg !22 {
entry:
  %self = alloca { float, float }, !dbg !25
  store { float, float } %0, { float, float }* %self, !dbg !25
  %other = alloca { float, float }, !dbg !25
  store { float, float } %1, { float, float }* %other, !dbg !25
  %s = getelementptr inbounds { float, float }, { float, float }* %self, i32 0, i32 0, !dbg !25
  %a = load float, float* %s, !dbg !25
  %s1 = getelementptr inbounds { float, float }, { float, float }* %other, i32 0, i32 1, !dbg !25
  %b = load float, float* %s1, !dbg !25
  %f32sum = fadd float %a, %b, !dbg !25
  ret float %f32sum, !dbg !25
}

; Function Attrs: noinline nounwind optnone
define i32 @main(i32 %0, i8** %1) local_unnamed_addr #0 {
entry:
  call void @_main(), !dbg !14
  ret i32 0, !dbg !14
}

attributes #0 = { noinline nounwind optnone }

!llvm.module.flags = !{!0}
!llvm.dbg.cu = !{!1}

!0 = !{i32 1, !"Debug Info Version", i32 1}
!1 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "Kestrel", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, enums: !3, splitDebugInlining: false)
!2 = !DIFile(filename: "program.ke", directory: ".")
!3 = !{}
!4 = distinct !DISubprogram(name: "main", linkageName: "_main", scope: null, file: !2, line: 15, type: !5, scopeLine: 15, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
!6 = !{!7}
!7 = !DIBasicType(name: "void", size: 16, flags: DIFlagPublic)
!8 = !DILocation(line: 15, scope: !9)
!9 = distinct !DILexicalBlock(scope: !4, file: !2, line: 15)
!10 = distinct !DISubprogram(name: "f", linkageName: "f", scope: null, file: !2, line: 45, type: !11, scopeLine: 45, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!11 = !DISubroutineType(flags: DIFlagPublic, types: !12)
!12 = !{!13, !13}
!13 = !DIBasicType(name: "float", size: 16, flags: DIFlagPublic)
!14 = !DILocation(line: 45, scope: !15)
!15 = distinct !DILexicalBlock(scope: !10, file: !2, line: 45)
!16 = distinct !DISubprogram(name: "a", linkageName: "a", scope: null, file: !2, line: 5, type: !17, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!17 = !DISubroutineType(flags: DIFlagPublic, types: !18)
!18 = !{!13, !19}
!19 = !DIBasicType(name: "{ float, float }", size: 16, flags: DIFlagPublic)
!20 = !DILocation(line: 5, scope: !21)
!21 = distinct !DILexicalBlock(scope: !16, file: !2, line: 5)
!22 = distinct !DISubprogram(name: "s.add", linkageName: "s.add", scope: null, file: !2, line: 10, type: !23, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!23 = !DISubroutineType(flags: DIFlagPublic, types: !24)
!24 = !{!13, !19, !19}
!25 = !DILocation(line: 10, column: 4, scope: !26)
!26 = distinct !DILexicalBlock(scope: !22, file: !2, line: 10, column: 4)
