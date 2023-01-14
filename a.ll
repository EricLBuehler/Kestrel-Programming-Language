; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone
define void @_main() local_unnamed_addr #0 !dbg !4 {
entry:
  %s = alloca {}, !dbg !8
  %st = alloca {}, !dbg !8
  store {} zeroinitializer, {}* %st, !dbg !8
  call void @f(i32 1, i32 2), !dbg !10
  ret void, !dbg !10
}

; Function Attrs: noinline nounwind optnone
define void @f(i32 %0, i32 %1) local_unnamed_addr #0 !dbg !12 {
entry:
  %a = alloca i32, !dbg !10
  store i32 %0, i32* %a, !dbg !10
  %b = alloca i32, !dbg !10
  store i32 %1, i32* %b, !dbg !10
  ret void, !dbg !10
}

; Function Attrs: noinline nounwind optnone
define i32 @main(i32 %0, i8** %1) local_unnamed_addr #0 {
entry:
  call void @_main(), !dbg !10
  ret i32 0, !dbg !10
}

attributes #0 = { noinline nounwind optnone }

!llvm.module.flags = !{!0}
!llvm.dbg.cu = !{!1}

!0 = !{i32 1, !"Debug Info Version", i32 3}
!1 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "Kestrel", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, enums: !3, splitDebugInlining: false)
!2 = !DIFile(filename: "program.ke", directory: ".")
!3 = !{}
!4 = distinct !DISubprogram(name: "main", linkageName: "_main", scope: null, file: !2, line: 8, type: !5, scopeLine: 8, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
!6 = !{!7}
!7 = !DIBasicType(name: "void", size: 16, flags: DIFlagPublic)
!8 = !DILocation(line: 8, scope: !9)
!9 = distinct !DILexicalBlock(scope: !4, file: !2, line: 8)
!10 = !DILocation(line: 4, scope: !11)
!11 = distinct !DILexicalBlock(scope: !12, file: !2, line: 4)
!12 = distinct !DISubprogram(name: "f", linkageName: "f", scope: null, file: !2, line: 4, type: !13, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!13 = !DISubroutineType(flags: DIFlagPublic, types: !14)
!14 = !{!7, !15, !15}
!15 = !DIBasicType(name: "i32", size: 16, flags: DIFlagPublic)
