; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

%enum_st_data = type opaque

; Function Attrs: noinline optnone
define void @_main() #0 !dbg !3 {
entry:
  %String = alloca { [7 x i8] }, !dbg !8
  %arr = getelementptr inbounds { [7 x i8] }, { [7 x i8] }* %String, i32 0, i32 0, !dbg !8
  store [7 x i8] c"Kestrel", [7 x i8]* %arr, !dbg !8
  %string = load { [7 x i8] }, { [7 x i8] }* %String, !dbg !8
  %str = alloca { [7 x i8] }, !dbg !8
  store { [7 x i8] } %string, { [7 x i8] }* %str, !dbg !8
  %arr1 = getelementptr inbounds { [7 x i8] }, { [7 x i8] }* %str, i32 0, i32 0, !dbg !8
  %load_arr = load [7 x i8], [7 x i8]* %arr1, !dbg !8
  br i1 false, label %then, label %else, !dbg !8

then:                                             ; preds = %entry
  %itmptr = getelementptr inbounds [7 x i8], [7 x i8]* %arr1, i32 0, i64 100, !dbg !8
  %item = load i8, i8* %itmptr, !dbg !8
  %enum_st = alloca { i32, %enum_st_data* }, !dbg !8
  %variant_id = getelementptr inbounds { i32, %enum_st_data* }, { i32, %enum_st_data* }* %enum_st, i32 0, i32 0, !dbg !8
  store i32 0, i32* %variant_id, !dbg !8
  %variant_data_ptr = alloca i8, !dbg !8
  store i8 %item, i8* %variant_data_ptr, !dbg !8
  %variant_data_bitcast = bitcast i8* %variant_data_ptr to %enum_st_data*, !dbg !8
  %variant_data = getelementptr inbounds { i32, %enum_st_data* }, { i32, %enum_st_data* }* %enum_st, i32 0, i32 1, !dbg !8
  store %enum_st_data* %variant_data_bitcast, %enum_st_data** %variant_data, !dbg !8
  br label %end, !dbg !8

else:                                             ; preds = %entry
  %enum_st2 = alloca { i32, %enum_st_data* }, !dbg !8
  %variant_id3 = getelementptr inbounds { i32, %enum_st_data* }, { i32, %enum_st_data* }* %enum_st2, i32 0, i32 0, !dbg !8
  store i32 1, i32* %variant_id3, !dbg !8
  br label %end, !dbg !8

end:                                              ; preds = %else, %then
  %check_phi = phi %enum_st_data [ %enum_st, %then ], [ %enum_st2, %else ], !dbg !8
}

declare void @_main.1()

attributes #0 = { noinline optnone }

!llvm.module.flags = !{!0}
!llvm.dbg.cu = !{!1}

!0 = !{i32 1, !"Debug Info Version", i32 3}
!1 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "Kestrel", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!2 = !DIFile(filename: "program.ke", directory: ".")
!3 = distinct !DISubprogram(name: "main", linkageName: "_main", scope: null, file: !2, type: !4, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !7)
!4 = !DISubroutineType(flags: DIFlagPublic, types: !5)
!5 = !{!6}
!6 = !DIBasicType(name: "void", size: 16, flags: DIFlagPublic)
!7 = <temporary!> !{}
!8 = !DILocation(line: 0, scope: !9)
!9 = distinct !DILexicalBlock(scope: !3, file: !2)
