	.text
	.file	"program.ke"
	.globl	_main                   # -- Begin function _main
	.p2align	4, 0x90
	.type	_main,@function
_main:                                  # @_main
# %bb.0:                                # %entry
	pushq	%rbp
	movq	%rsp, %rbp
	pushq	%r15
	pushq	%r14
	pushq	%rbx
	subq	$72, %rsp
	movl	$1067282596, -60(%rbp)  # imm = 0x3F9D70A4
	movss	-60(%rbp), %xmm0        # xmm0 = mem[0],zero,zero,zero
	callq	f
	movb	$-123, -29(%rbp)
	movb	$-90, -30(%rbp)
	movb	$-97, -31(%rbp)
	movb	$-16, -32(%rbp)
	movb	$32, -33(%rbp)
	movb	$108, -34(%rbp)
	movb	$101, -35(%rbp)
	movb	$114, -36(%rbp)
	movb	$116, -37(%rbp)
	movb	$115, -38(%rbp)
	movb	$101, -39(%rbp)
	movb	$75, -40(%rbp)
	movb	-40(%rbp), %sil
	movb	-39(%rbp), %dil
	movb	-38(%rbp), %r8b
	movb	-37(%rbp), %r9b
	movb	-36(%rbp), %r10b
	movb	-35(%rbp), %r11b
	movb	-34(%rbp), %r14b
	movb	-33(%rbp), %r15b
	movb	-32(%rbp), %dl
	movb	-31(%rbp), %bl
	movb	-30(%rbp), %al
	movb	-29(%rbp), %cl
	movb	%cl, -45(%rbp)
	movb	%al, -46(%rbp)
	movb	%bl, -47(%rbp)
	movb	%dl, -48(%rbp)
	movb	%r15b, -49(%rbp)
	movb	%r14b, -50(%rbp)
	movb	%r11b, -51(%rbp)
	movb	%r10b, -52(%rbp)
	movb	%r9b, -53(%rbp)
	movb	%r8b, -54(%rbp)
	movb	%dil, -55(%rbp)
	movb	%sil, -56(%rbp)
	movl	$129413, -76(%rbp)      # imm = 0x1F985
	movss	-60(%rbp), %xmm0        # xmm0 = mem[0],zero,zero,zero
	mulss	-60(%rbp), %xmm0
	cvttss2si	%xmm0, %eax
	movl	%eax, -68(%rbp)
	movl	$2, -68(%rbp)
	leaq	-68(%rbp), %rax
	movq	%rax, -88(%rbp)
	movl	$12, -72(%rbp)
# %bb.1:                                # %if
	movq	%rsp, %rax
	addq	$-16, %rax
	movq	%rax, %rsp
	movl	$100, (%rax)
# %bb.2:                                # %if_end
	leaq	-24(%rbp), %rsp
	popq	%rbx
	popq	%r14
	popq	%r15
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
