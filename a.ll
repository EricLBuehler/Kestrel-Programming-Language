; ModuleID = 'module'
source_filename = "program.ke"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone
define void @_main() local_unnamed_addr #0 {
entry:
  %x = alloca i32
  store i32 3, i32* %x
  %x1 = load i32, i32* %x
  %i32sum = add i32 %x1, 3
  %y = alloca i32
  store i32 %i32sum, i32* %y
  ret void
}

; Function Attrs: noinline nounwind optnone
define i32 @main(i32 %0, i8** %1) local_unnamed_addr #0 {
entry:
  call void @_main()
  ret i32 0
}

attributes #0 = { noinline nounwind optnone }
