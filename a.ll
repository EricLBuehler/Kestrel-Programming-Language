; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone
define void @_main() local_unnamed_addr #0 !dbg !4 {
entry:
  %enum_st3 = alloca { i32, i32 }, !dbg !8
  %enum_st = alloca { i32, i32 }, !dbg !8
  %arr = alloca [3 x i32], !dbg !8
  %i32 = getelementptr [3 x i32], [3 x i32]* %arr, i8 0, i8 0, !dbg !8
  store i32 1, i32* %i32, !dbg !8
  %i321 = getelementptr [3 x i32], [3 x i32]* %arr, i8 0, i8 1, !dbg !8
  store i32 2, i32* %i321, !dbg !8
  %i322 = getelementptr [3 x i32], [3 x i32]* %arr, i8 0, i8 2, !dbg !8
  store i32 3, i32* %i322, !dbg !8
  br label %then, !dbg !8

then:                                             ; preds = %entry
  %itmptr = getelementptr inbounds [3 x i32], [3 x i32]* %arr, i32 0, i64 1, !dbg !8
  store i32 2, i32* %itmptr, !dbg !8
  %variant_id = getelementptr inbounds { i32, i32 }, { i32, i32 }* %enum_st, i32 0, i32 0, !dbg !8
  store i32 0, i32* %variant_id, !dbg !8
  br label %end, !dbg !8

end:                                              ; preds = %then
  %some_case = load { i32, i32 }, { i32, i32 }* %enum_st, !dbg !8
  %none_case = load { i32, i32 }, { i32, i32 }* %enum_st3, !dbg !8
  ret void, !dbg !8
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
!4 = distinct !DISubprogram(name: "main", linkageName: "_main", scope: null, file: !2, type: !5, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
!6 = !{!7}
!7 = !DIBasicType(name: "void", size: 16, flags: DIFlagPublic)
!8 = !DILocation(line: 0, scope: !9)
!9 = distinct !DILexicalBlock(scope: !4, file: !2)
