; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone
define void @f(float %0, float %1) local_unnamed_addr #0 !dbg !4 {
entry:
  %x = alloca float, !dbg !9
  store float %0, float* %x, !dbg !9
  %y = alloca float, !dbg !9
  store float %1, float* %y, !dbg !9
  %2 = bitcast float* %x to i8*
  tail call void @free(i8* %2), !dbg !9
  %3 = bitcast float* %y to i8*
  tail call void @free(i8* %3), !dbg !9
  ret void, !dbg !9
}

; Function Attrs: nounwind
declare void @free(i8* nocapture) local_unnamed_addr #1

; Function Attrs: noinline nounwind optnone
define void @_main() local_unnamed_addr #0 !dbg !11 {
entry:
  %x = alloca float, !dbg !14
  store float 0x3FF3AE1480000000, float* %x, !dbg !14
  %x1 = load float, float* %x, !dbg !14
  call void @f(float %x1, float 1.000000e+00), !dbg !14
  %0 = bitcast float* %x to i8*
  tail call void @free(i8* %0), !dbg !14
  ret void, !dbg !14
}

; Function Attrs: noinline nounwind optnone
define i32 @main(i32 %0, i8** %1) local_unnamed_addr #0 {
entry:
  call void @_main(), !dbg !14
  ret i32 0, !dbg !14
}

attributes #0 = { noinline nounwind optnone }
attributes #1 = { nounwind }

!llvm.module.flags = !{!0}
!llvm.dbg.cu = !{!1}

!0 = !{i32 1, !"Debug Info Version", i32 1}
!1 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "Kestrel", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, enums: !3, splitDebugInlining: false)
!2 = !DIFile(filename: "program.ke", directory: ".")
!3 = !{}
!4 = distinct !DISubprogram(name: "f", linkageName: "f", scope: null, file: !2, type: !5, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
!6 = !{!7, !8, !8}
!7 = !DIBasicType(name: "void", size: 16, flags: DIFlagPublic)
!8 = !DIBasicType(name: "float", size: 16, flags: DIFlagPublic)
!9 = !DILocation(line: 0, scope: !10)
!10 = distinct !DILexicalBlock(scope: !4, file: !2)
!11 = distinct !DISubprogram(name: "main", linkageName: "_main", scope: null, file: !2, line: 4, type: !12, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!12 = !DISubroutineType(flags: DIFlagPublic, types: !13)
!13 = !{!7}
!14 = !DILocation(line: 4, scope: !15)
!15 = distinct !DILexicalBlock(scope: !11, file: !2, line: 4)
