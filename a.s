	.text
	.file	"program.ke"
	.globl	_main                   # -- Begin function _main
	.p2align	4, 0x90
	.type	_main,@function
_main:                                  # @_main
# %bb.0:                                # %entry
	subq	$40, %rsp
	movl	$1067282596, (%rsp)     # imm = 0x3F9D70A4
	movss	(%rsp), %xmm0           # xmm0 = mem[0],zero,zero,zero
	callq	f
	movb	$-123, 15(%rsp)
	movb	$-90, 14(%rsp)
	movb	$-97, 13(%rsp)
	movb	$-16, 12(%rsp)
	movb	$32, 11(%rsp)
	movb	$108, 10(%rsp)
	movb	$101, 9(%rsp)
	movb	$114, 8(%rsp)
	movb	$116, 7(%rsp)
	movb	$115, 6(%rsp)
	movb	$101, 5(%rsp)
	movb	$75, 4(%rsp)
	movl	$129413, 28(%rsp)       # imm = 0x1F985
	movss	(%rsp), %xmm0           # xmm0 = mem[0],zero,zero,zero
	mulss	(%rsp), %xmm0
	cvttss2si	%xmm0, %eax
	movl	%eax, 16(%rsp)
	movl	$2, 16(%rsp)
	leaq	16(%rsp), %rax
	movq	%rax, 32(%rsp)
	movl	$2, 24(%rsp)
	addq	$40, %rsp
	retq
.Lfunc_end0:
	.size	_main, .Lfunc_end0-_main
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2               # -- Begin function f
.LCPI1_0:
	.long	1065353216              # float 1
	.text
	.globl	f
	.p2align	4, 0x90
	.type	f,@function
f:                                      # @f
# %bb.0:                                # %entry
	subq	$24, %rsp
	movss	%xmm0, 20(%rsp)
	movss	20(%rsp), %xmm0         # xmm0 = mem[0],zero,zero,zero
	movss	.LCPI1_0(%rip), %xmm1   # xmm1 = mem[0],zero,zero,zero
	movaps	%xmm0, %xmm2
	addss	%xmm1, %xmm2
	movss	%xmm2, 8(%rsp)
	movss	%xmm0, 12(%rsp)
	movss	8(%rsp), %xmm0          # xmm0 = mem[0],zero,zero,zero
	movss	12(%rsp), %xmm1         # xmm1 = mem[0],zero,zero,zero
	movss	%xmm1, 4(%rsp)
	movss	%xmm0, (%rsp)
	movl	$1094713344, (%rsp)     # imm = 0x41400000
	movss	(%rsp), %xmm0           # xmm0 = mem[0],zero,zero,zero
	movss	4(%rsp), %xmm1          # xmm1 = mem[0],zero,zero,zero
	callq	s.a
	addq	$24, %rsp
	retq
.Lfunc_end1:
	.size	f, .Lfunc_end1-f
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2               # -- Begin function s.a
.LCPI2_0:
	.long	1120403456              # float 100
	.text
	.globl	s.a
	.p2align	4, 0x90
	.type	s.a,@function
s.a:                                    # @s.a
# %bb.0:                                # %entry
	movss	.LCPI2_0(%rip), %xmm2   # xmm2 = mem[0],zero,zero,zero
	movss	%xmm0, -8(%rsp)
	movss	%xmm1, -4(%rsp)
	mulss	-8(%rsp), %xmm2
	movaps	%xmm2, %xmm0
	retq
.Lfunc_end2:
	.size	s.a, .Lfunc_end2-s.a
                                        # -- End function
	.globl	sadd                    # -- Begin function sadd
	.p2align	4, 0x90
	.type	sadd,@function
sadd:                                   # @sadd
# %bb.0:                                # %entry
	movss	%xmm0, -8(%rsp)
	movss	%xmm1, -4(%rsp)
	movss	%xmm3, -12(%rsp)
	movss	%xmm2, -16(%rsp)
	movss	-8(%rsp), %xmm0         # xmm0 = mem[0],zero,zero,zero
	addss	-12(%rsp), %xmm0
	retq
.Lfunc_end3:
	.size	sadd, .Lfunc_end3-sadd
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
.Lfunc_end4:
	.size	main, .Lfunc_end4-main
                                        # -- End function
	.section	".note.GNU-stack","",@progbits
