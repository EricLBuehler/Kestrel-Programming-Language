; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone
define { float } @f(float %0) local_unnamed_addr #0 !dbg !4 {
entry:
  %x = alloca float, !dbg !9
  store float %0, float* %x, !dbg !9
  %x1 = load float, float* %x, !dbg !9
  %s = alloca { float }, !dbg !9
  %a = getelementptr inbounds { float }, { float }* %s, i32 0, i32 0, !dbg !9
  store float %x1, float* %a, !dbg !9
  %y = alloca { float }*, !dbg !9
  store { float }* %s, { float }** %y, !dbg !9
  %y2 = load { float }*, { float }** %y, !dbg !9
  ret { float }* %y2, !dbg !9
}

; Function Attrs: noinline nounwind optnone
define void @_main() local_unnamed_addr #0 !dbg !11 {
entry:
  %x = alloca float, !dbg !15
  store float 0x3FF3AE1480000000, float* %x, !dbg !15
  %x1 = load float, float* %x, !dbg !15
  %res = call { float } @f(float %x1), !dbg !15
  ret void, !dbg !15
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
!4 = distinct !DISubprogram(name: "f", linkageName: "f", scope: null, file: !2, line: 4, type: !5, scopeLine: 4, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
!6 = !{!7, !8}
!7 = !DIBasicType(name: "{ float }", size: 16, flags: DIFlagPublic)
!8 = !DIBasicType(name: "float", size: 16, flags: DIFlagPublic)
!9 = !DILocation(line: 4, scope: !10)
!10 = distinct !DILexicalBlock(scope: !4, file: !2, line: 4)
!11 = distinct !DISubprogram(name: "main", linkageName: "_main", scope: null, file: !2, line: 11, type: !12, scopeLine: 11, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!12 = !DISubroutineType(flags: DIFlagPublic, types: !13)
!13 = !{!14}
!14 = !DIBasicType(name: "void", size: 16, flags: DIFlagPublic)
!15 = !DILocation(line: 11, scope: !16)
!16 = distinct !DILexicalBlock(scope: !11, file: !2, line: 11)
