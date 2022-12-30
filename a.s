	.text
	.file	"program.ke"
	.globl	_main                   # -- Begin function _main
	.p2align	4, 0x90
	.type	_main,@function
_main:                                  # @_main
# %bb.0:                                # %entry
	subq	$40, %rsp
	movl	$1067282596, 4(%rsp)    # imm = 0x3F9D70A4
	movss	4(%rsp), %xmm0          # xmm0 = mem[0],zero,zero,zero
	callq	f
	movb	$-123, 19(%rsp)
	movb	$-90, 18(%rsp)
	movb	$-97, 17(%rsp)
	movb	$-16, 16(%rsp)
	movb	$32, 15(%rsp)
	movb	$108, 14(%rsp)
	movb	$101, 13(%rsp)
	movb	$114, 12(%rsp)
	movb	$116, 11(%rsp)
	movb	$115, 10(%rsp)
	movb	$101, 9(%rsp)
	movb	$75, 8(%rsp)
	movl	$129413, 28(%rsp)       # imm = 0x1F985
	movss	4(%rsp), %xmm0          # xmm0 = mem[0],zero,zero,zero
	mulss	4(%rsp), %xmm0
	cvttss2si	%xmm0, %eax
	movl	%eax, 20(%rsp)
	movl	$2, 20(%rsp)
	leaq	20(%rsp), %rax
	movq	%rax, 32(%rsp)
	addq	$40, %rsp
	retq
.Lfunc_end0:
	.size	_main, .Lfunc_end0-_main
                                        # -- End function
	.globl	f                       # -- Begin function f
	.p2align	4, 0x90
	.type	f,@function
f:                                      # @f
# %bb.0:                                # %entry
	movss	%xmm0, -4(%rsp)
	movss	-4(%rsp), %xmm0         # xmm0 = mem[0],zero,zero,zero
	movss	%xmm0, -8(%rsp)
	movss	-8(%rsp), %xmm0         # xmm0 = mem[0],zero,zero,zero
	movss	%xmm0, -16(%rsp)
	movl	$1094713344, -16(%rsp)  # imm = 0x41400000
	movss	-16(%rsp), %xmm0        # xmm0 = mem[0],zero,zero,zero
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
