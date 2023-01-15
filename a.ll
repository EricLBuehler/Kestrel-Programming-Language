; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone
define void @_main() local_unnamed_addr #0 !dbg !4 {
entry:
  %s = alloca { i32 }, !dbg !8
  %x = getelementptr inbounds { i32 }, { i32 }* %s, i32 0, i32 0, !dbg !8
  store i32 10, i32* %x, !dbg !8
  %s1 = load { i32 }, { i32 }* %s, !dbg !8
  %st = alloca { i32 }, !dbg !8
  store { i32 } %s1, { i32 }* %st, !dbg !8
  ret void, !dbg !8
}

; Function Attrs: noinline nounwind optnone
define void @s.func(i64 %0, i32 %1) local_unnamed_addr #0 !dbg !10 {
entry:
  %a = alloca i64, !dbg !15
  store i64 %0, i64* %a, !dbg !15
  %b = alloca i32, !dbg !15
  store i32 %1, i32* %b, !dbg !15
  ret void, !dbg !15
}

; Function Attrs: noinline nounwind optnone
define i32 @main(i32 %0, i8** %1) local_unnamed_addr #0 {
entry:
  call void @_main(), !dbg !8
  ret i32 0, !dbg !8
}

attributes #0 = { noinline nounwind optnone }

!llvm.module.flags = !{!0}
!llvm.dbg.cu = !{!1}

!0 = !{i32 1, !"Debug Info Version", i32 3}
!1 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "Kestrel", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, enums: !3, splitDebugInlining: false)
!2 = !DIFile(filename: "program.ke", directory: ".")
!3 = !{}
!4 = distinct !DISubprogram(name: "main", linkageName: "_main", scope: null, file: !2, line: 15, type: !5, scopeLine: 15, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
!6 = !{!7}
!7 = !DIBasicType(name: "void", size: 16, flags: DIFlagPublic)
!8 = !DILocation(line: 15, scope: !9)
!9 = distinct !DILexicalBlock(scope: !4, file: !2, line: 15)
!10 = distinct !DISubprogram(name: "s.func", linkageName: "s.func", scope: null, file: !2, line: 10, type: !11, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!11 = !DISubroutineType(flags: DIFlagPublic, types: !12)
!12 = !{!7, !13, !14}
!13 = !DIBasicType(name: "i64", size: 16, flags: DIFlagPublic)
!14 = !DIBasicType(name: "i32", size: 16, flags: DIFlagPublic)
!15 = !DILocation(line: 10, column: 4, scope: !16)
!16 = distinct !DILexicalBlock(scope: !10, file: !2, line: 10, column: 4)
