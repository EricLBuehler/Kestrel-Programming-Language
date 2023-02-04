; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone
define void @f({ i32 }* %0) local_unnamed_addr #0 !dbg !4 {
entry:
  %s = getelementptr inbounds { i32 }, { i32 }* %0, i32 0, i32 0, !dbg !9
  store i32 100, i32* %s, !dbg !9
  ret void, !dbg !9
}

; Function Attrs: noinline nounwind optnone
define void @_main() local_unnamed_addr #0 !dbg !11 {
entry:
  %s = alloca { i32 }, !dbg !14
  %a = getelementptr inbounds { i32 }, { i32 }* %s, i32 0, i32 0, !dbg !14
  store i32 10, i32* %a, !dbg !14
  %s1 = load { i32 }, { i32 }* %s, !dbg !14
  %a2 = alloca { i32 }, !dbg !14
  store { i32 } %s1, { i32 }* %a2, !dbg !14
  call void @f({ i32 }* %a2), !dbg !14
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

!0 = !{i32 1, !"Debug Info Version", i32 3}
!1 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "Kestrel", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, enums: !3, splitDebugInlining: false)
!2 = !DIFile(filename: "program.ke", directory: ".")
!3 = !{}
!4 = distinct !DISubprogram(name: "f", linkageName: "f", scope: null, file: !2, line: 3, type: !5, scopeLine: 3, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
!6 = !{!7, !8}
!7 = !DIBasicType(name: "void", size: 16, flags: DIFlagPublic)
!8 = !DIBasicType(name: "{ i32 }*", size: 16, flags: DIFlagPublic)
!9 = !DILocation(line: 3, scope: !10)
!10 = distinct !DILexicalBlock(scope: !4, file: !2, line: 3)
!11 = distinct !DISubprogram(name: "main", linkageName: "_main", scope: null, file: !2, line: 6, type: !12, scopeLine: 6, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!12 = !DISubroutineType(flags: DIFlagPublic, types: !13)
!13 = !{!7}
!14 = !DILocation(line: 6, scope: !15)
!15 = distinct !DILexicalBlock(scope: !11, file: !2, line: 6)
