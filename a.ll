; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

%enum_st_data = type opaque

; Function Attrs: noinline nounwind optnone
define void @_main() local_unnamed_addr #0 !dbg !4 {
entry:
  %enum_st = alloca { i32, %enum_st_data* }, !dbg !8
  %variant_id = getelementptr inbounds { i32, %enum_st_data* }, { i32, %enum_st_data* }* %enum_st, i32 0, i32 0, !dbg !8
  store i32 0, i32* %variant_id, !dbg !8
  %variant_data_ptr = alloca i32, !dbg !8
  store i32 100, i32* %variant_data_ptr, !dbg !8
  %variant_data_bitcast = bitcast i32* %variant_data_ptr to %enum_st_data*, !dbg !8
  %variant_data = getelementptr inbounds { i32, %enum_st_data* }, { i32, %enum_st_data* }* %enum_st, i32 0, i32 1, !dbg !8
  store %enum_st_data* %variant_data_bitcast, %enum_st_data** %variant_data, !dbg !8
  %opt = alloca { i32, %enum_st_data* }*, !dbg !8
  store { i32, %enum_st_data* }* %enum_st, { i32, %enum_st_data* }** %opt, !dbg !8
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
