	.text
	.file	"program.ke"
	.globl	_main                   # -- Begin function _main
	.p2align	4, 0x90
	.type	_main,@function
_main:                                  # @_main
# %bb.0:                                # %entry
	pushq	%rbp
	pushq	%r14
	pushq	%rbx
	subq	$64, %rsp
	movl	$1067282596, 36(%rsp)   # imm = 0x3F9D70A4
	movss	36(%rsp), %xmm0         # xmm0 = mem[0],zero,zero,zero
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
	movb	8(%rsp), %sil
	movb	9(%rsp), %dil
	movb	10(%rsp), %r8b
	movb	11(%rsp), %r9b
	movb	12(%rsp), %r10b
	movb	13(%rsp), %r11b
	movb	14(%rsp), %bpl
	movb	15(%rsp), %r14b
	movb	16(%rsp), %dl
	movb	17(%rsp), %bl
	movb	18(%rsp), %al
	movb	19(%rsp), %cl
	movb	%cl, 35(%rsp)
	movb	%al, 34(%rsp)
	movb	%bl, 33(%rsp)
	movb	%dl, 32(%rsp)
	movb	%r14b, 31(%rsp)
	movb	%bpl, 30(%rsp)
	movb	%r11b, 29(%rsp)
	movb	%r10b, 28(%rsp)
	movb	%r9b, 27(%rsp)
	movb	%r8b, 26(%rsp)
	movb	%dil, 25(%rsp)
	movb	%sil, 24(%rsp)
	movl	$129413, 52(%rsp)       # imm = 0x1F985
	movss	36(%rsp), %xmm0         # xmm0 = mem[0],zero,zero,zero
	mulss	36(%rsp), %xmm0
	cvttss2si	%xmm0, %eax
	movl	%eax, 40(%rsp)
	movl	$2, 40(%rsp)
	leaq	40(%rsp), %rax
	movq	%rax, 56(%rsp)
	movl	$2, 48(%rsp)
	addq	$64, %rsp
	popq	%rbx
	popq	%r14
	popq	%rbp
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
	.globl	s.add                   # -- Begin function s.add
	.p2align	4, 0x90
	.type	s.add,@function
s.add:                                  # @s.add
# %bb.0:                                # %entry
	movss	%xmm0, -8(%rsp)
	movss	%xmm1, -4(%rsp)
	movss	%xmm3, -12(%rsp)
	movss	%xmm2, -16(%rsp)
	movss	-8(%rsp), %xmm0         # xmm0 = mem[0],zero,zero,zero
	addss	-12(%rsp), %xmm0
	retq
.Lfunc_end3:
	.size	s.add, .Lfunc_end3-s.add
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
