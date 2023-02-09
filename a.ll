; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: nofree nounwind
declare i32 @printf(i8* nocapture readonly, ...) local_unnamed_addr #0

; Function Attrs: noinline noreturn nounwind optnone
define void @_main() local_unnamed_addr #1 !dbg !4 {
entry:
  %inplace_ptr = alloca { [15 x i8] }, !dbg !8
  %String = alloca { [15 x i8] }, !dbg !8
  br label %loop_head, !dbg !8

loop_head:                                        ; preds = %loop_then, %entry
  br label %loop_then, !dbg !8

loop_then:                                        ; preds = %loop_head
  %arr = getelementptr inbounds { [15 x i8] }, { [15 x i8] }* %String, i32 0, i32 0, !dbg !8
  store [15 x i8] c"Hello, world!\0A\00", [15 x i8]* %arr, !dbg !8
  %string = load { [15 x i8] }, { [15 x i8] }* %String, !dbg !8
  store { [15 x i8] } %string, { [15 x i8] }* %inplace_ptr, !dbg !8
  %data = getelementptr inbounds { [15 x i8] }, { [15 x i8] }* %inplace_ptr, i32 0, i32 0, !dbg !8
  %data_ptr = getelementptr inbounds [15 x i8], [15 x i8]* %data, i32 0, i32 0, !dbg !8
  %printf_call = call i32 (i8*, ...) @printf(i8* %data_ptr), !dbg !8
  br label %loop_head, !dbg !8
}

; Function Attrs: noinline noreturn nounwind optnone
define i32 @main(i32 %0, i8** %1) local_unnamed_addr #1 {
entry:
  call void @_main(), !dbg !8
  unreachable
}

attributes #0 = { nofree nounwind }
attributes #1 = { noinline noreturn nounwind optnone }

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
