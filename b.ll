; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline optnone
define void @_main() #0 !dbg !3 {
entry:
  ret void, !dbg !8
}

; Function Attrs: noinline optnone
define i32 @g() #0 !dbg !10 {
entry:
  ret i32 0, !dbg !15
}

declare void @_main.1()

declare i32 @g.2()

attributes #0 = { noinline optnone }

!llvm.module.flags = !{!0}
!llvm.dbg.cu = !{!1}

!0 = !{i32 1, !"Debug Info Version", i32 1}
!1 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "Kestrel", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!2 = !DIFile(filename: "program.ke", directory: ".")
!3 = distinct !DISubprogram(name: "main", linkageName: "_main", scope: null, file: !2, line: 4, type: !4, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !7)
!4 = !DISubroutineType(flags: DIFlagPublic, types: !5)
!5 = !{!6}
!6 = !DIBasicType(name: "void", size: 16, flags: DIFlagPublic)
!7 = <temporary!> !{}
!8 = !DILocation(line: 4, scope: !9)
!9 = distinct !DILexicalBlock(scope: !3, file: !2, line: 4)
!10 = distinct !DISubprogram(name: "g", linkageName: "g", scope: null, file: !2, line: 7, type: !11, scopeLine: 7, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !14)
!11 = !DISubroutineType(flags: DIFlagPublic, types: !12)
!12 = !{!13}
!13 = !DIBasicType(name: "i32", size: 16, flags: DIFlagPublic)
!14 = <temporary!> !{}
!15 = !DILocation(line: 7, scope: !16)
!16 = distinct !DILexicalBlock(scope: !10, file: !2, line: 7)
