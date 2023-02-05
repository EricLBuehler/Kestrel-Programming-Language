; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

%enum_st_data = type opaque

; Function Attrs: noinline nounwind optnone
define void @_main() local_unnamed_addr #0 !dbg !4 {
entry:
  %String = alloca { [7 x i8] }, !dbg !8
  %arr = getelementptr inbounds { [7 x i8] }, { [7 x i8] }* %String, i32 0, i32 0, !dbg !8
  store [7 x i8] c"Kestrel", [7 x i8]* %arr, !dbg !8
  %string = load { [7 x i8] }, { [7 x i8] }* %String, !dbg !8
  %str = alloca { [7 x i8] }, !dbg !8
  store { [7 x i8] } %string, { [7 x i8] }* %str, !dbg !8
  %arr1 = getelementptr inbounds { [7 x i8] }, { [7 x i8] }* %str, i32 0, i32 0, !dbg !8
  %load_arr = load [7 x i8], [7 x i8]* %arr1, !dbg !8
  br label %else, !dbg !8

else:                                             ; preds = %entry
  %enum_st2 = alloca { i32, %enum_st_data* }, !dbg !8
  %variant_id3 = getelementptr inbounds { i32, %enum_st_data* }, { i32, %enum_st_data* }* %enum_st2, i32 0, i32 0, !dbg !8
  store i32 1, i32* %variant_id3, !dbg !8
  br label %end, !dbg !8

end:                                              ; preds = %else
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
