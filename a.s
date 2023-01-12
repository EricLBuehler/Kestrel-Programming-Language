	.text
	.file	"program.ke"
	.globl	_main                   # -- Begin function _main
	.p2align	4, 0x90
	.type	_main,@function
_main:                                  # @_main
.Lfunc_begin0:
	.file	1 "./program.ke"
	.loc	1 20 0                  # program.ke:20:0
	.cfi_sections .debug_frame
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
.Ltmp0:
	pushq	%r15
	pushq	%r14
	pushq	%rbx
	subq	$88, %rsp
	.cfi_offset %rbx, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.loc	1 20 0 prologue_end     # program.ke:20:0
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
	movl	$129413, -84(%rbp)      # imm = 0x1F985
	movss	-60(%rbp), %xmm0        # xmm0 = mem[0],zero,zero,zero
	mulss	-60(%rbp), %xmm0
	cvttss2si	%xmm0, %eax
	movl	%eax, -80(%rbp)
	movl	$2, -80(%rbp)
	leaq	-80(%rbp), %rax
	movq	%rax, -104(%rbp)
	movq	$12, -72(%rbp)
	movb	-55(%rbp), %al
	movb	%al, -61(%rbp)
# %bb.1:                                # %if
	movq	%rsp, %rax
	addq	$-16, %rax
	movq	%rax, %rsp
	movl	$100, (%rax)
	movq	$1, -96(%rbp)
# %bb.2:                                # %if_end
	jmp	.LBB0_3
.LBB0_3:                                # %loop_head
                                        # =>This Inner Loop Header: Depth=1
	.loc	1 0 0 is_stmt 0         # program.ke:0:0
	movb	$1, %al
	.loc	1 20 0                  # program.ke:20:0
	testb	%al, %al
	jne	.LBB0_5
	jmp	.LBB0_4
.LBB0_4:                                # %loop_then
                                        #   in Loop: Header=BB0_3 Depth=1
	movq	-72(%rbp), %rax
	addq	$1, %rax
	movq	%rax, -72(%rbp)
	jmp	.LBB0_3
.LBB0_5:                                # %loop_end
	leaq	-24(%rbp), %rsp
	popq	%rbx
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Ltmp1:
.Lfunc_end0:
	.size	_main, .Lfunc_end0-_main
	.cfi_endproc
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
.Lfunc_begin1:
	.loc	1 51 0 is_stmt 1        # program.ke:51:0
	.cfi_startproc
# %bb.0:                                # %entry
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
.Ltmp2:
	.loc	1 51 0 prologue_end     # program.ke:51:0
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
	.cfi_def_cfa_offset 8
	retq
.Ltmp3:
.Lfunc_end1:
	.size	f, .Lfunc_end1-f
	.cfi_endproc
                                        # -- End function
	.globl	g                       # -- Begin function g
	.p2align	4, 0x90
	.type	g,@function
g:                                      # @g
.Lfunc_begin2:
	.loc	1 62 0                  # program.ke:62:0
	.cfi_startproc
# %bb.0:                                # %entry
	.loc	1 62 0 prologue_end     # program.ke:62:0
	xorl	%eax, %eax
	retq
.Ltmp4:
.Lfunc_end2:
	.size	g, .Lfunc_end2-g
	.cfi_endproc
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2               # -- Begin function s.a
.LCPI3_0:
	.long	1120403456              # float 100
	.text
	.globl	s.a
	.p2align	4, 0x90
	.type	s.a,@function
s.a:                                    # @s.a
.Lfunc_begin3:
	.loc	1 10 0                  # program.ke:10:0
	.cfi_startproc
# %bb.0:                                # %entry
	movss	.LCPI3_0(%rip), %xmm2   # xmm2 = mem[0],zero,zero,zero
.Ltmp5:
	.loc	1 10 0 prologue_end     # program.ke:10:0
	movss	%xmm0, -8(%rsp)
	movss	%xmm1, -4(%rsp)
	mulss	-8(%rsp), %xmm2
	movaps	%xmm2, %xmm0
	retq
.Ltmp6:
.Lfunc_end3:
	.size	s.a, .Lfunc_end3-s.a
	.cfi_endproc
                                        # -- End function
	.globl	s.add                   # -- Begin function s.add
	.p2align	4, 0x90
	.type	s.add,@function
s.add:                                  # @s.add
.Lfunc_begin4:
	.loc	1 15 0                  # program.ke:15:0
	.cfi_startproc
# %bb.0:                                # %entry
	.loc	1 15 4 prologue_end     # program.ke:15:4
	movss	%xmm0, -8(%rsp)
	movss	%xmm1, -4(%rsp)
	movss	%xmm3, -12(%rsp)
	movss	%xmm2, -16(%rsp)
	movss	-8(%rsp), %xmm0         # xmm0 = mem[0],zero,zero,zero
	addss	-12(%rsp), %xmm0
	retq
.Ltmp7:
.Lfunc_end4:
	.size	s.add, .Lfunc_end4-s.add
	.cfi_endproc
                                        # -- End function
	.globl	main                    # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
.Lfunc_begin5:
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
                                        # kill: killed $rsi
                                        # kill: killed $edi
	callq	_main
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end5:
	.size	main, .Lfunc_end5-main
	.cfi_endproc
                                        # -- End function
	.section	.debug_str,"MS",@progbits,1
.Linfo_string0:
	.asciz	"Kestrel"               # string offset=0
.Linfo_string1:
	.asciz	"program.ke"            # string offset=8
.Linfo_string2:
	.asciz	"."                     # string offset=19
.Linfo_string3:
	.asciz	"_main"                 # string offset=21
.Linfo_string4:
	.asciz	"main"                  # string offset=27
.Linfo_string5:
	.asciz	"void"                  # string offset=32
.Linfo_string6:
	.asciz	"f"                     # string offset=37
.Linfo_string7:
	.asciz	"float"                 # string offset=39
.Linfo_string8:
	.asciz	"g"                     # string offset=45
.Linfo_string9:
	.asciz	"i32"                   # string offset=47
.Linfo_string10:
	.asciz	"a"                     # string offset=51
.Linfo_string11:
	.asciz	"s.add"                 # string offset=53
	.section	.debug_abbrev,"",@progbits
	.byte	1                       # Abbreviation Code
	.byte	17                      # DW_TAG_compile_unit
	.byte	1                       # DW_CHILDREN_yes
	.byte	37                      # DW_AT_producer
	.byte	14                      # DW_FORM_strp
	.byte	19                      # DW_AT_language
	.byte	5                       # DW_FORM_data2
	.byte	3                       # DW_AT_name
	.byte	14                      # DW_FORM_strp
	.byte	16                      # DW_AT_stmt_list
	.byte	23                      # DW_FORM_sec_offset
	.byte	27                      # DW_AT_comp_dir
	.byte	14                      # DW_FORM_strp
	.ascii	"\264B"                 # DW_AT_GNU_pubnames
	.byte	25                      # DW_FORM_flag_present
	.byte	17                      # DW_AT_low_pc
	.byte	1                       # DW_FORM_addr
	.byte	18                      # DW_AT_high_pc
	.byte	6                       # DW_FORM_data4
	.byte	0                       # EOM(1)
	.byte	0                       # EOM(2)
	.byte	2                       # Abbreviation Code
	.byte	46                      # DW_TAG_subprogram
	.byte	0                       # DW_CHILDREN_no
	.byte	17                      # DW_AT_low_pc
	.byte	1                       # DW_FORM_addr
	.byte	18                      # DW_AT_high_pc
	.byte	6                       # DW_FORM_data4
	.byte	64                      # DW_AT_frame_base
	.byte	24                      # DW_FORM_exprloc
	.byte	110                     # DW_AT_linkage_name
	.byte	14                      # DW_FORM_strp
	.byte	3                       # DW_AT_name
	.byte	14                      # DW_FORM_strp
	.byte	58                      # DW_AT_decl_file
	.byte	11                      # DW_FORM_data1
	.byte	59                      # DW_AT_decl_line
	.byte	11                      # DW_FORM_data1
	.byte	73                      # DW_AT_type
	.byte	19                      # DW_FORM_ref4
	.byte	50                      # DW_AT_accessibility
	.byte	11                      # DW_FORM_data1
	.byte	0                       # EOM(1)
	.byte	0                       # EOM(2)
	.byte	3                       # Abbreviation Code
	.byte	36                      # DW_TAG_base_type
	.byte	0                       # DW_CHILDREN_no
	.byte	3                       # DW_AT_name
	.byte	14                      # DW_FORM_strp
	.byte	62                      # DW_AT_encoding
	.byte	11                      # DW_FORM_data1
	.byte	11                      # DW_AT_byte_size
	.byte	11                      # DW_FORM_data1
	.byte	0                       # EOM(1)
	.byte	0                       # EOM(2)
	.byte	0                       # EOM(3)
	.section	.debug_info,"",@progbits
.Lcu_begin0:
	.long	.Ldebug_info_end0-.Ldebug_info_start0 # Length of Unit
.Ldebug_info_start0:
	.short	4                       # DWARF version number
	.long	.debug_abbrev           # Offset Into Abbrev. Section
	.byte	8                       # Address Size (in bytes)
	.byte	1                       # Abbrev [1] 0xb:0xcb DW_TAG_compile_unit
	.long	.Linfo_string0          # DW_AT_producer
	.short	2                       # DW_AT_language
	.long	.Linfo_string1          # DW_AT_name
	.long	.Lline_table_start0     # DW_AT_stmt_list
	.long	.Linfo_string2          # DW_AT_comp_dir
                                        # DW_AT_GNU_pubnames
	.quad	.Lfunc_begin0           # DW_AT_low_pc
	.long	.Lfunc_end4-.Lfunc_begin0 # DW_AT_high_pc
	.byte	2                       # Abbrev [2] 0x2a:0x1e DW_TAG_subprogram
	.quad	.Lfunc_begin0           # DW_AT_low_pc
	.long	.Lfunc_end0-.Lfunc_begin0 # DW_AT_high_pc
	.byte	1                       # DW_AT_frame_base
	.byte	86
	.long	.Linfo_string3          # DW_AT_linkage_name
	.long	.Linfo_string4          # DW_AT_name
	.byte	1                       # DW_AT_decl_file
	.byte	20                      # DW_AT_decl_line
	.long	192                     # DW_AT_type
	.byte	1                       # DW_AT_accessibility
                                        # DW_ACCESS_public
	.byte	2                       # Abbrev [2] 0x48:0x1e DW_TAG_subprogram
	.quad	.Lfunc_begin1           # DW_AT_low_pc
	.long	.Lfunc_end1-.Lfunc_begin1 # DW_AT_high_pc
	.byte	1                       # DW_AT_frame_base
	.byte	87
	.long	.Linfo_string6          # DW_AT_linkage_name
	.long	.Linfo_string6          # DW_AT_name
	.byte	1                       # DW_AT_decl_file
	.byte	51                      # DW_AT_decl_line
	.long	199                     # DW_AT_type
	.byte	1                       # DW_AT_accessibility
                                        # DW_ACCESS_public
	.byte	2                       # Abbrev [2] 0x66:0x1e DW_TAG_subprogram
	.quad	.Lfunc_begin2           # DW_AT_low_pc
	.long	.Lfunc_end2-.Lfunc_begin2 # DW_AT_high_pc
	.byte	1                       # DW_AT_frame_base
	.byte	87
	.long	.Linfo_string8          # DW_AT_linkage_name
	.long	.Linfo_string8          # DW_AT_name
	.byte	1                       # DW_AT_decl_file
	.byte	62                      # DW_AT_decl_line
	.long	206                     # DW_AT_type
	.byte	1                       # DW_AT_accessibility
                                        # DW_ACCESS_public
	.byte	2                       # Abbrev [2] 0x84:0x1e DW_TAG_subprogram
	.quad	.Lfunc_begin3           # DW_AT_low_pc
	.long	.Lfunc_end3-.Lfunc_begin3 # DW_AT_high_pc
	.byte	1                       # DW_AT_frame_base
	.byte	87
	.long	.Linfo_string10         # DW_AT_linkage_name
	.long	.Linfo_string10         # DW_AT_name
	.byte	1                       # DW_AT_decl_file
	.byte	10                      # DW_AT_decl_line
	.long	199                     # DW_AT_type
	.byte	1                       # DW_AT_accessibility
                                        # DW_ACCESS_public
	.byte	2                       # Abbrev [2] 0xa2:0x1e DW_TAG_subprogram
	.quad	.Lfunc_begin4           # DW_AT_low_pc
	.long	.Lfunc_end4-.Lfunc_begin4 # DW_AT_high_pc
	.byte	1                       # DW_AT_frame_base
	.byte	87
	.long	.Linfo_string11         # DW_AT_linkage_name
	.long	.Linfo_string11         # DW_AT_name
	.byte	1                       # DW_AT_decl_file
	.byte	15                      # DW_AT_decl_line
	.long	199                     # DW_AT_type
	.byte	1                       # DW_AT_accessibility
                                        # DW_ACCESS_public
	.byte	3                       # Abbrev [3] 0xc0:0x7 DW_TAG_base_type
	.long	.Linfo_string5          # DW_AT_name
	.byte	0                       # DW_AT_encoding
	.byte	2                       # DW_AT_byte_size
	.byte	3                       # Abbrev [3] 0xc7:0x7 DW_TAG_base_type
	.long	.Linfo_string7          # DW_AT_name
	.byte	0                       # DW_AT_encoding
	.byte	2                       # DW_AT_byte_size
	.byte	3                       # Abbrev [3] 0xce:0x7 DW_TAG_base_type
	.long	.Linfo_string9          # DW_AT_name
	.byte	0                       # DW_AT_encoding
	.byte	2                       # DW_AT_byte_size
	.byte	0                       # End Of Children Mark
.Ldebug_info_end0:
	.section	.debug_pubnames,"",@progbits
	.long	.LpubNames_end0-.LpubNames_begin0 # Length of Public Names Info
.LpubNames_begin0:
	.short	2                       # DWARF Version
	.long	.Lcu_begin0             # Offset of Compilation Unit Info
	.long	214                     # Compilation Unit Length
	.long	132                     # DIE offset
	.asciz	"a"                     # External Name
	.long	42                      # DIE offset
	.asciz	"main"                  # External Name
	.long	72                      # DIE offset
	.asciz	"f"                     # External Name
	.long	102                     # DIE offset
	.asciz	"g"                     # External Name
	.long	162                     # DIE offset
	.asciz	"s.add"                 # External Name
	.long	0                       # End Mark
.LpubNames_end0:
	.section	.debug_pubtypes,"",@progbits
	.long	.LpubTypes_end0-.LpubTypes_begin0 # Length of Public Types Info
.LpubTypes_begin0:
	.short	2                       # DWARF Version
	.long	.Lcu_begin0             # Offset of Compilation Unit Info
	.long	214                     # Compilation Unit Length
	.long	192                     # DIE offset
	.asciz	"void"                  # External Name
	.long	199                     # DIE offset
	.asciz	"float"                 # External Name
	.long	206                     # DIE offset
	.asciz	"i32"                   # External Name
	.long	0                       # End Mark
.LpubTypes_end0:
	.section	".note.GNU-stack","",@progbits
	.section	.debug_line,"",@progbits
.Lline_table_start0:
