; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

%st_data = type opaque

@vtables = local_unnamed_addr constant { { void ({ i32, %st_data* }, i32)* } } { { void ({ i32, %st_data* }, i32)* } { void ({ i32, %st_data* }, i32)* @s.func } }

; Function Attrs: noinline nounwind optnone
define void @s.func({ i32, %st_data* } %0, i32 %1) #0 !dbg !4 {
entry:
  ret void, !dbg !10
}

; Function Attrs: noinline optnone
define void @_main() local_unnamed_addr #1 !dbg !12 {
entry:
  %s = alloca { i32 }, !dbg !15
  %x = getelementptr inbounds { i32 }, { i32 }* %s, i32 0, i32 0, !dbg !15
  store i32 10, i32* %x, !dbg !15
  %s1 = load { i32 }, { i32 }* %s, !dbg !15
  %st = alloca { i32 }, !dbg !15
  store { i32 } %s1, { i32 }* %st, !dbg !15
  %st2 = load { i32 }, { i32 }* %st, !dbg !15
  %x3 = alloca { i32, %st_data* }, !dbg !15
  %idptr = getelementptr inbounds { i32, %st_data* }, { i32, %st_data* }* %x3, i32 0, i32 0, !dbg !15
  store i32 0, i32* %idptr, !dbg !15
  %item = getelementptr inbounds { i32, %st_data* }, { i32, %st_data* }* %x3, i32 0, i32 1, !dbg !15
  %struct_ptr = alloca { i32 }, !dbg !15
  store { i32 } %st2, { i32 }* %struct_ptr, !dbg !15
  %st_bitcast = bitcast { i32 }* %struct_ptr to %st_data*, !dbg !15
  store %st_data* %st_bitcast, %st_data** %item, !dbg !15
  %id_ptr = getelementptr inbounds { i32, %st_data* }, { i32, %st_data* }* %x3, i32 0, i32 0, !dbg !15
  %id = load i32, i32* %id_ptr, !dbg !15
  %vtable = getelementptr inbounds { { void ({ i32, %st_data* }, i32)* } }, { { void ({ i32, %st_data* }, i32)* } }* @vtables, i32 %id, i32 0, !dbg !15
  %method_ptr = getelementptr inbounds { void ({ i32, %st_data* }, i32)* }, { void ({ i32, %st_data* }, i32)* }* %vtable, i32 0, i32 0, !dbg !15
  %method = load void ({ i32, %st_data* }, i32)*, void ({ i32, %st_data* }, i32)** %method_ptr, !dbg !15
  %instance = load { i32, %st_data* }, { i32, %st_data* }* %x3, !dbg !15
  call void %method({ i32, %st_data* } %instance, i32 2), !dbg !15
  %String = alloca { [7 x i8] }, !dbg !15
  %arr = getelementptr inbounds { [7 x i8] }, { [7 x i8] }* %String, i32 0, i32 0, !dbg !15
  store [7 x i8] c"Kestrel", [7 x i8]* %arr, !dbg !15
  %string = load { [7 x i8] }, { [7 x i8] }* %String, !dbg !15
  %var = alloca { [7 x i8] }, !dbg !15
  store { [7 x i8] } %string, { [7 x i8] }* %var, !dbg !15
  ret void, !dbg !15
}

; Function Attrs: noinline optnone
define i32 @main(i32 %0, i8** %1) local_unnamed_addr #1 {
entry:
  call void @_main(), !dbg !15
  ret i32 0, !dbg !15
}

attributes #0 = { noinline nounwind optnone }
attributes #1 = { noinline optnone }

!llvm.module.flags = !{!0}
!llvm.dbg.cu = !{!1}

!0 = !{i32 1, !"Debug Info Version", i32 3}
!1 = distinct !DICompileUnit(language: DW_LANG_C, file: !2, producer: "Kestrel", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, enums: !3, splitDebugInlining: false)
!2 = !DIFile(filename: "program.ke", directory: ".")
!3 = !{}
!4 = distinct !DISubprogram(name: "s.func", linkageName: "s.func", scope: null, file: !2, line: 10, type: !5, scopeLine: 10, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!5 = !DISubroutineType(flags: DIFlagPublic, types: !6)
!6 = !{!7, !8, !9}
!7 = !DIBasicType(name: "void", size: 16, flags: DIFlagPublic)
!8 = !DIBasicType(name: "{ i32, %st_data* }", size: 16, flags: DIFlagPublic)
!9 = !DIBasicType(name: "i32", size: 16, flags: DIFlagPublic)
!10 = !DILocation(line: 10, column: 4, scope: !11)
!11 = distinct !DILexicalBlock(scope: !4, file: !2, line: 10, column: 4)
!12 = distinct !DISubprogram(name: "main", linkageName: "_main", scope: null, file: !2, line: 20, type: !13, scopeLine: 20, flags: DIFlagPublic, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !1, retainedNodes: !3)
!13 = !DISubroutineType(flags: DIFlagPublic, types: !14)
!14 = !{!7}
!15 = !DILocation(line: 20, scope: !16)
!16 = distinct !DILexicalBlock(scope: !12, file: !2, line: 20)
