; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone
define void @_main() local_unnamed_addr #0 !dbg !4 {
entry:
  %x = alloca float, !dbg !8
  store float 0x3FF3AE1480000000, float* %x, !dbg !8
  %x1 = load float, float* %x, !dbg !8
  %res = call { float, float } @f(float %x1), !dbg !8
  %name = alloca [12 x i8], !dbg !8
  store [12 x i8] c"Kestrel \F0\9F\A6\85", [12 x i8]* %name, !dbg !8
  %char = alloca i32, !dbg !8
  store i32 129413, i32* %char, !dbg !8
  %x2 = load float, float* %x, !dbg !8
  %x3 = load float, float* %x, !dbg !8
  %f32mul = fmul float %x2, %x3, !dbg !8
  %ftoi = fptosi float %f32mul to i32, !dbg !8
  %arr = alloca [2 x i32], !dbg !8
  %i32 = getelementptr [2 x i32], [2 x i32]* %arr, i8 0, i8 0, !dbg !8
  store i32 %ftoi, i32* %i32, !dbg !8
  %i324 = getelementptr [2 x i32], [2 x i32]* %arr, i8 0, i8 0, !dbg !8
  store i32 2, i32* %i324, !dbg !8
  %arr5 = alloca [2 x i32]*, !dbg !8
  store [2 x i32]* %arr, [2 x i32]** %arr5, !dbg !8
  %len = alloca i32, !dbg !8
  store i32 2, i32* %len, !dbg !8
  ret void, !dbg !8
}

; Function Attrs: noinline nounwind optnone
define { float, float } @f(float %0) local_unnamed_addr #0 !dbg !10 {
entry:
  %x = alloca float, !dbg !15
  store float %0, float* %x, !dbg !15
  %x1 = load float, float* %x, !dbg !15
  %f32sum = fadd float %x1, 1.000000e+00, !dbg !15
  %x2 = load float, float* %x, !dbg !15
  %s = alloca { float, float }, !dbg !15
  %a = getelementptr inbounds { float, float }, { float, float }* %s, i32 0, i32 0, !dbg !15
  store float %f32sum, float* %a, !dbg !15
  %b = getelementptr inbounds { float, float }, { float, float }* %s, i32 0, i32 1, !dbg !15
  store float %x2, float* %b, !dbg !15
  %s3 = load { float, float }, { float, float }* %s, !dbg !15
  %y = alloca { float, float }, !dbg !15
  store { float, float } %s3, { float, float }* %y, !dbg !15
  %s4 = getelementptr inbounds { float, float }, { float, float }* %y, i32 0, i32 0, !dbg !15
  store float 1.200000e+01, float* %s4, !dbg !15
  %s5 = load { float, float }, { float, float }* %y, !dbg !15
  call void @s.a({ float, float } %s5), !dbg !15
  %y6 = load { float, float }, { float, float }* %y, !dbg !15
  ret { float, float } %y6, !dbg !15
}

; Function Attrs: noinline nounwind optnone
define void @s.a({ float, float } %0) local_unnamed_addr #0 !dbg !17 {
entry:
  %self = alloca { float, float }, !dbg !20
  store { float, float } %0, { float, float }* %self, !dbg !20
  ret void, !dbg !20
}

; Function Attrs: noinline nounwind optnone
define float @sadd({ float, float } %0, { float, float } %1) local_unnamed_addr #0 !dbg !22 {
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
  call void @_main(), !dbg !15
  ret i32 0, !dbg !15
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
!10 = distinct !DISubprogram(name: "f", linkageName: "f", scope: null, file: !2, line: 25, type: !11, scopeLine: 25, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!11 = !DISubroutineType(flags: DIFlagPublic, types: !12)
!12 = !{!13, !14}
!13 = !DIBasicType(name: "{ float, float }", size: 16, flags: DIFlagPublic)
!14 = !DIBasicType(name: "float", size: 16, flags: DIFlagPublic)
!15 = !DILocation(line: 25, scope: !16)
!16 = distinct !DILexicalBlock(scope: !10, file: !2, line: 25)
!17 = distinct !DISubprogram(name: "a", linkageName: "a", scope: null, file: !2, line: 5, type: !18, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!18 = !DISubroutineType(flags: DIFlagPublic, types: !19)
!19 = !{!7, !13}
!20 = !DILocation(line: 5, scope: !21)
!21 = distinct !DILexicalBlock(scope: !17, file: !2, line: 5)
!22 = distinct !DISubprogram(name: "sadd", linkageName: "sadd", scope: null, file: !2, line: 10, type: !23, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!23 = !DISubroutineType(flags: DIFlagPublic, types: !24)
!24 = !{!14, !13, !13}
!25 = !DILocation(line: 10, column: 4, scope: !26)
!26 = distinct !DILexicalBlock(scope: !22, file: !2, line: 10, column: 4)
