	.text
	.file	"program.ke"
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2               # -- Begin function _main
.LCPI0_0:
	.long	1067282596              # float 1.23000002
	.text
	.globl	_main
	.p2align	4, 0x90
	.type	_main,@function
_main:                                  # @_main
# %bb.0:                                # %entry
	pushq	%rax
	movss	.LCPI0_0(%rip), %xmm0   # xmm0 = mem[0],zero,zero,zero
	movss	%xmm0, 4(%rsp)
	movss	4(%rsp), %xmm0          # xmm0 = mem[0],zero,zero,zero
	callq	f
	popq	%rax
	retq
.Lfunc_end0:
	.size	_main, .Lfunc_end0-_main
                                        # -- End function
	.globl	f                       # -- Begin function f
	.p2align	4, 0x90
	.type	f,@function
f:                                      # @f
# %bb.0:                                # %entry
	movss	%xmm0, -12(%rsp)
	movss	-12(%rsp), %xmm0        # xmm0 = mem[0],zero,zero,zero
	movss	%xmm0, -16(%rsp)
	leaq	-16(%rsp), %rax
	movq	%rax, -8(%rsp)
	movq	-8(%rsp), %rax
	movl	$1094713344, (%rax)     # imm = 0x41400000
	movq	-8(%rsp), %rax
	movss	(%rax), %xmm0           # xmm0 = mem[0],zero,zero,zero
	retq
.Lfunc_end1:
	.size	f, .Lfunc_end1-f
                                        # -- End function
	.globl	main                    # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
# %bb.0:                                # %entry
	pushq	%rax
                                        # kill: killed $rsi
                                        # kill: killed $edi
	callq	_main
	xorl	%eax, %eax
	popq	%rcx
	retq
.Lfunc_end2:
	.size	main, .Lfunc_end2-main
                                        # -- End function
	.section	".note.GNU-stack","",@progbits
