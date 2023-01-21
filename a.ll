; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

%st_data = type opaque

@vtable = local_unnamed_addr constant { { void ({ i32, %st_data* }, i32)* } } { { void ({ i32, %st_data* }, i32)* } { void ({ i32, %st_data* }, i32)* @s.func } }

; Function Attrs: noinline optnone
define void @_main() local_unnamed_addr #0 !dbg !4 {
entry:
  %s = alloca { i32 }, !dbg !8
  %x = getelementptr inbounds { i32 }, { i32 }* %s, i32 0, i32 0, !dbg !8
  store i32 10, i32* %x, !dbg !8
  %s1 = load { i32 }, { i32 }* %s, !dbg !8
  %st = alloca { i32 }, !dbg !8
  store { i32 } %s1, { i32 }* %st, !dbg !8
  %st2 = load { i32 }, { i32 }* %st, !dbg !8
  %x3 = alloca { i32, %st_data* }, !dbg !8
  %idptr = getelementptr inbounds { i32, %st_data* }, { i32, %st_data* }* %x3, i32 0, i32 0, !dbg !8
  store i32 0, i32* %idptr, !dbg !8
  %item = getelementptr inbounds { i32, %st_data* }, { i32, %st_data* }* %x3, i32 0, i32 1, !dbg !8
  %malloccall = tail call i8* @malloc(i32 ptrtoint (i32* getelementptr (i32, i32* null, i32 1) to i32))
  %struct_ptr = bitcast i8* %malloccall to { i32 }*, !dbg !8
  store { i32 } %st2, { i32 }* %struct_ptr, !dbg !8
  %st_bitcast = bitcast { i32 }* %struct_ptr to %st_data*, !dbg !8
  store %st_data* %st_bitcast, %st_data** %item, !dbg !8
  %id_ptr = getelementptr inbounds { i32, %st_data* }, { i32, %st_data* }* %x3, i32 0, i32 0, !dbg !8
  %id = load i32, i32* %id_ptr, !dbg !8
  %vtable = getelementptr inbounds { { void ({ i32, %st_data* }, i32)* } }, { { void ({ i32, %st_data* }, i32)* } }* @vtable, i32 %id, i32 0, !dbg !8
  %method_ptr = getelementptr inbounds { void ({ i32, %st_data* }, i32)* }, { void ({ i32, %st_data* }, i32)* }* %vtable, i32 0, i32 0, !dbg !8
  %method = load void ({ i32, %st_data* }, i32)*, void ({ i32, %st_data* }, i32)** %method_ptr, !dbg !8
  %instance = load { i32, %st_data* }, { i32, %st_data* }* %x3, !dbg !8
  call void %method({ i32, %st_data* } %instance, i32 2), !dbg !8
  %free_dyn = getelementptr inbounds { i32, %st_data* }, { i32, %st_data* }* %x3, i32 0, i32 1, !dbg !8
  %0 = bitcast %st_data** %free_dyn to i8*
  tail call void @free(i8* %0), !dbg !8
  ret void, !dbg !8
}

; Function Attrs: noinline nounwind optnone
define void @s.func({ i32, %st_data* } %0, i32 %1) #1 !dbg !10 {
entry:
  ret void, !dbg !15
}

; Function Attrs: nofree nounwind
declare noalias i8* @malloc(i32) local_unnamed_addr #2

; Function Attrs: nounwind
declare void @free(i8* nocapture) local_unnamed_addr #3

; Function Attrs: noinline optnone
define i32 @main(i32 %0, i8** %1) local_unnamed_addr #0 {
entry:
  call void @_main(), !dbg !8
  ret i32 0, !dbg !8
}

attributes #0 = { noinline optnone }
attributes #1 = { noinline nounwind optnone }
attributes #2 = { nofree nounwind }
attributes #3 = { nounwind }

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
!13 = !DIBasicType(name: "{ i32, %st_data* }", size: 16, flags: DIFlagPublic)
!14 = !DIBasicType(name: "i32", size: 16, flags: DIFlagPublic)
!15 = !DILocation(line: 10, column: 4, scope: !16)
!16 = distinct !DILexicalBlock(scope: !10, file: !2, line: 10, column: 4)
