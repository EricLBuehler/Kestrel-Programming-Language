; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: nofree nounwind
declare i32 @printf(i8* nocapture readonly, ...) local_unnamed_addr #0

; Function Attrs: noinline nounwind optnone
define void @_main() local_unnamed_addr #1 !dbg !4 {
entry:
  %format = alloca [3 x i8], !dbg !8
  %String = alloca { [20 x i8] }, !dbg !8
  %inplace_ptr = alloca i128, !dbg !8
  store i128 123456789, i128* %inplace_ptr, !dbg !8
  %bitcast_value = bitcast i128* %inplace_ptr to <{ i64, i64 }>*, !dbg !8
  %lo_ptr = getelementptr inbounds <{ i64, i64 }>, <{ i64, i64 }>* %bitcast_value, i32 0, i32 0, !dbg !8
  %lo = load i64, i64* %lo_ptr, !dbg !8
  %hi_ptr = getelementptr inbounds <{ i64, i64 }>, <{ i64, i64 }>* %bitcast_value, i32 0, i32 1, !dbg !8
  %hi = load i64, i64* %hi_ptr, !dbg !8
  %arr = getelementptr inbounds { [20 x i8] }, { [20 x i8] }* %String, i32 0, i32 0, !dbg !8
  %data_ptr = getelementptr inbounds [20 x i8], [20 x i8]* %arr, i32 0, i32 0, !dbg !8
  store [3 x i8] c"%u\00", [3 x i8]* %format, !dbg !8
  %data_ptr1 = getelementptr inbounds [3 x i8], [3 x i8]* %format, i32 0, i32 0, !dbg !8
  %sprintf_call = call i32 (i8*, i8*, ...) @sprintf(i8* %data_ptr, i8* %data_ptr1, i64 %lo, i64 %hi), !dbg !8
  %data = getelementptr inbounds { [20 x i8] }, { [20 x i8] }* %String, i32 0, i32 0, !dbg !8
  %data_ptr2 = getelementptr inbounds [20 x i8], [20 x i8]* %data, i32 0, i32 0, !dbg !8
  %printf_call = call i32 (i8*, ...) @printf(i8* %data_ptr2), !dbg !8
  ret void, !dbg !8
}

; Function Attrs: nofree nounwind
declare i32 @sprintf(i8* noalias nocapture, i8* nocapture readonly, ...) local_unnamed_addr #0

; Function Attrs: noinline nounwind optnone
define i32 @main(i32 %0, i8** %1) local_unnamed_addr #1 {
entry:
  call void @_main(), !dbg !8
  ret i32 0, !dbg !8
}

attributes #0 = { nofree nounwind }
attributes #1 = { noinline nounwind optnone }

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
