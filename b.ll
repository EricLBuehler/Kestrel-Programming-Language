; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline optnone
define void @_main() #0 !dbg !3 {
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
  br i1 true, label %if, label %else_if, !dbg !8

if:                                               ; preds = %entry
  %x9 = alloca i32, !dbg !8
  store i32 100, i32* %x9, !dbg !8
  store i64 1, i64* %uninit, !dbg !8
  br label %if_end, !dbg !8

if10:                                             ; preds = %else, %else_if
  %z = alloca i32, !dbg !8
  store i32 200, i32* %z, !dbg !8
  store i64 2, i64* %uninit, !dbg !8
  br label %if_end, !dbg !8

else:                                             ; preds = %else_if
  %y = alloca i32, !dbg !8
  store i32 300, i32* %y, !dbg !8
  store i64 3, i64* %uninit, !dbg !8
  br label %if10, !dbg !8

if_end:                                           ; preds = %if10, %if
  br label %loop_head, !dbg !8

else_if:                                          ; preds = %entry
  br i1 true, label %if10, label %else, !dbg !8

loop_head:                                        ; preds = %loop_then, %if_end
  %len11 = load i64, i64* %len, !dbg !8
  %i8lt = icmp uge i64 %len11, %len11, !dbg !8
  %bool = icmp ne i1 %i8lt, false, !dbg !8
  br i1 %bool, label %loop_then, label %loop_end, !dbg !8

loop_then:                                        ; preds = %loop_head
  %len12 = load i64, i64* %len, !dbg !8
  %u64sum = add i64 %len12, 1, !dbg !8
  store i64 %u64sum, i64* %len, !dbg !8
  br label %loop_head, !dbg !8

loop_end:                                         ; preds = %loop_head
}

declare float @f(float)

; Function Attrs: noinline optnone
define float @s.a({ float, float } %0) #0 !dbg !10 {
entry:
  %self = alloca { float, float }, !dbg !16
  store { float, float } %0, { float, float }* %self, !dbg !16
  %s = getelementptr inbounds { float, float }, { float, float }* %self, i32 0, i32 0, !dbg !16
  %a = load float, float* %s, !dbg !16
  %f32mul = fmul float %a, 1.000000e+02, !dbg !16
  ret float %f32mul, !dbg !16
}

; Function Attrs: noinline optnone
define float @s.add({ float, float } %0, { float, float } %1) #0 !dbg !18 {
entry:
  %self = alloca { float, float }, !dbg !22
  store { float, float } %0, { float, float }* %self, !dbg !22
  %other = alloca { float, float }, !dbg !22
  store { float, float } %1, { float, float }* %other, !dbg !22
  %s = getelementptr inbounds { float, float }, { float, float }* %self, i32 0, i32 0, !dbg !22
  %a = load float, float* %s, !dbg !22
  %s1 = getelementptr inbounds { float, float }, { float, float }* %other, i32 0, i32 1, !dbg !22
  %b = load float, float* %s1, !dbg !22
  %f32sum = fadd float %a, %b, !dbg !22
  ret float %f32sum, !dbg !22
}

declare void @_main.1()

attributes #0 = { noinline optnone }

!llvm.module.flags = !{!0}
!llvm.dbg.cu = !{!1}

!0 = !{i32 1, !"Debug Info Version", i32 1}
!1 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "Kestrel", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!2 = !DIFile(filename: "program.ke", directory: ".")
!3 = distinct !DISubprogram(name: "main", linkageName: "_main", scope: null, file: !2, line: 15, type: !4, scopeLine: 15, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !7)
!4 = !DISubroutineType(flags: DIFlagPublic, types: !5)
!5 = !{!6}
!6 = !DIBasicType(name: "void", size: 16, flags: DIFlagPublic)
!7 = <temporary!> !{}
!8 = !DILocation(line: 15, scope: !9)
!9 = distinct !DILexicalBlock(scope: !3, file: !2, line: 15)
!10 = distinct !DISubprogram(name: "a", linkageName: "a", scope: null, file: !2, line: 5, type: !11, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !15)
!11 = !DISubroutineType(flags: DIFlagPublic, types: !12)
!12 = !{!13, !14}
!13 = !DIBasicType(name: "float", size: 16, flags: DIFlagPublic)
!14 = !DIBasicType(name: "{ float, float }", size: 16, flags: DIFlagPublic)
!15 = <temporary!> !{}
!16 = !DILocation(line: 5, scope: !17)
!17 = distinct !DILexicalBlock(scope: !10, file: !2, line: 5)
!18 = distinct !DISubprogram(name: "s.add", linkageName: "s.add", scope: null, file: !2, line: 10, type: !19, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !21)
!19 = !DISubroutineType(flags: DIFlagPublic, types: !20)
!20 = !{!13, !14, !14}
!21 = <temporary!> !{}
!22 = !DILocation(line: 10, column: 4, scope: !23)
!23 = distinct !DILexicalBlock(scope: !18, file: !2, line: 10, column: 4)
