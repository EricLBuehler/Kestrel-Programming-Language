; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone
define float @f(float %0) local_unnamed_addr #0 !dbg !4 {
entry:
  %x = alloca float, !dbg !8
  store float %0, float* %x, !dbg !8
  %y = alloca float, !dbg !8
  store float 1.000000e+02, float* %y, !dbg !8
  %y1 = load float, float* %y, !dbg !8
  ret float %y1, !dbg !8
}

; Function Attrs: noinline nounwind optnone
define void @_main() local_unnamed_addr #0 !dbg !10 {
entry:
  %x = alloca i32, !dbg !14
  store i32 1000000, i32* %x, !dbg !14
  %res = call float @f(float 1.000000e+02), !dbg !14
  ret void, !dbg !14
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
!4 = distinct !DISubprogram(name: "f", linkageName: "f", scope: null, file: !2, type: !5, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
!6 = !{!7, !7}
!7 = !DIBasicType(name: "float", size: 16, flags: DIFlagPublic)
!8 = !DILocation(line: 0, scope: !9)
!9 = distinct !DILexicalBlock(scope: !4, file: !2)
!10 = distinct !DISubprogram(name: "main", linkageName: "_main", scope: null, file: !2, line: 5, type: !11, scopeLine: 5, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!11 = !DISubroutineType(flags: DIFlagPublic, types: !12)
!12 = !{!13}
!13 = !DIBasicType(name: "void", size: 16, flags: DIFlagPublic)
!14 = !DILocation(line: 5, scope: !15)
!15 = distinct !DILexicalBlock(scope: !10, file: !2, line: 5)
