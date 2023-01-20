; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

%st_data = type opaque

@vtable = constant { { void ({ i32 }, i32)* } } { { void ({ i32 }, i32)* } { void ({ i32 }, i32)* @s.func } }

; Function Attrs: noinline optnone
define void @_main() #0 !dbg !3 {
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
  %instance_ptr = getelementptr inbounds { i32, %st_data* }, { i32, %st_data* }* %x3, i32 0, i32 1, !dbg !8
  %id_ptr = getelementptr inbounds { i32, %st_data* }, { i32, %st_data* }* %x3, i32 0, i32 0, !dbg !8
  %id = load i32, i32* %id_ptr, !dbg !8
  %vtable = getelementptr inbounds { { void ({ i32 }, i32)* } }, { { void ({ i32 }, i32)* } }* @vtable, i32 %id, i32 0, !dbg !8
  %vtable4 = getelementptr inbounds { void ({ i32 }, i32)* }, { void ({ i32 }, i32)* }* %vtable, i32 0, i32 0, !dbg !8
}

; Function Attrs: noinline optnone
define void @s.func({ i32 } %0, i32 %1) #0 !dbg !10 {
entry:
  ret void, !dbg !16
}

declare void @_main.1()

declare noalias i8* @malloc(i32)

attributes #0 = { noinline optnone }

!llvm.module.flags = !{!0}
!llvm.dbg.cu = !{!1}

!0 = !{i32 1, !"Debug Info Version", i32 3}
!1 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "Kestrel", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!2 = !DIFile(filename: "program.ke", directory: ".")
!3 = distinct !DISubprogram(name: "main", linkageName: "_main", scope: null, file: !2, line: 15, type: !4, scopeLine: 15, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !7)
!4 = !DISubroutineType(flags: DIFlagPublic, types: !5)
!5 = !{!6}
!6 = !DIBasicType(name: "void", size: 16, flags: DIFlagPublic)
!7 = <temporary!> !{}
!8 = !DILocation(line: 15, scope: !9)
!9 = distinct !DILexicalBlock(scope: !3, file: !2, line: 15)
!10 = distinct !DISubprogram(name: "s.func", linkageName: "s.func", scope: null, file: !2, line: 10, type: !11, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !15)
!11 = !DISubroutineType(flags: DIFlagPublic, types: !12)
!12 = !{!6, !13, !14}
!13 = !DIBasicType(name: "{ i32 }", size: 16, flags: DIFlagPublic)
!14 = !DIBasicType(name: "i32", size: 16, flags: DIFlagPublic)
!15 = <temporary!> !{}
!16 = !DILocation(line: 10, column: 4, scope: !17)
!17 = distinct !DILexicalBlock(scope: !10, file: !2, line: 10, column: 4)
