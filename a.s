	.text
	.file	"program.ke"
	.globl	s.func                  # -- Begin function s.func
	.p2align	4, 0x90
	.type	s.func,@function
s.func:                                 # @s.func
.Lfunc_begin0:
	.file	1 "./program.ke"
	.loc	1 10 0                  # program.ke:10:0
	.cfi_startproc
# %bb.0:                                # %entry
	.loc	1 10 4 prologue_end     # program.ke:10:4
	retq
.Ltmp0:
.Lfunc_end0:
	.size	s.func, .Lfunc_end0-s.func
	.cfi_endproc
                                        # -- End function
	.globl	_main                   # -- Begin function _main
	.p2align	4, 0x90
	.type	_main,@function
_main:                                  # @_main
.Lfunc_begin1:
	.loc	1 20 0                  # program.ke:20:0
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
.Ltmp1:
	pushq	%r15
	pushq	%r14
	pushq	%rbx
	subq	$104, %rsp
	.cfi_offset %rbx, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.loc	1 20 0 prologue_end     # program.ke:20:0
	movl	$10, -80(%rbp)
	movl	-80(%rbp), %eax
	movl	%eax, -72(%rbp)
	movl	-72(%rbp), %eax
	movl	$0, -104(%rbp)
	movl	%eax, -64(%rbp)
	leaq	-64(%rbp), %rax
	movq	%rax, -96(%rbp)
	movslq	-104(%rbp), %rdi
	movq	vtables(,%rdi,8), %rax
	movq	-96(%rbp), %rsi
                                        # kill: def $edi killed $edi killed $rdi
	movl	$2, %edx
	callq	*%rax
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
	movl	$1, -120(%rbp)
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
	leaq	-56(%rbp), %rax
	movq	%rax, -112(%rbp)
	leaq	-120(%rbp), %rax
	movq	%rax, -88(%rbp)
	movq	-88(%rbp), %rax
	movl	(%rax), %eax
	cmpl	$1, %eax
	jne	.LBB1_2
# %bb.1:                                # %pattern_0
	movl	$123, %eax
	jmp	.LBB1_5
.LBB1_2:                                # %pattern_check_1
	cmpl	$0, %eax
	jne	.LBB1_4
# %bb.3:                                # %pattern_1
	movl	$146, %eax
	jmp	.LBB1_5
.LBB1_4:                                # %default
	movl	$456, %eax              # imm = 0x1C8
	jmp	.LBB1_5
.LBB1_5:                                # %end
	movq	%rsp, %rcx
	addq	$-16, %rcx
	movq	%rcx, %rsp
	movl	%eax, (%rcx)
	leaq	-24(%rbp), %rsp
	popq	%rbx
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.Ltmp2:
.Lfunc_end1:
	.size	_main, .Lfunc_end1-_main
	.cfi_endproc
                                        # -- End function
	.globl	main                    # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
.Lfunc_begin2:
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
.Lfunc_end2:
	.size	main, .Lfunc_end2-main
	.cfi_endproc
                                        # -- End function
	.type	vtables,@object         # @vtables
	.section	.rodata,"a",@progbits
	.globl	vtables
	.p2align	3
vtables:
	.quad	s.func
	.size	vtables, 8

	.section	.debug_str,"MS",@progbits,1
.Linfo_string0:
	.asciz	"Kestrel"               # string offset=0
.Linfo_string1:
	.asciz	"program.ke"            # string offset=8
.Linfo_string2:
	.asciz	"."                     # string offset=19
.Linfo_string3:
	.asciz	"s.func"                # string offset=21
.Linfo_string4:
	.asciz	"void"                  # string offset=28
.Linfo_string5:
	.asciz	"_main"                 # string offset=33
.Linfo_string6:
	.asciz	"main"                  # string offset=39
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
	.byte	1                       # Abbrev [1] 0xb:0x63 DW_TAG_compile_unit
	.long	.Linfo_string0          # DW_AT_producer
	.short	2                       # DW_AT_language
	.long	.Linfo_string1          # DW_AT_name
	.long	.Lline_table_start0     # DW_AT_stmt_list
	.long	.Linfo_string2          # DW_AT_comp_dir
                                        # DW_AT_GNU_pubnames
	.quad	.Lfunc_begin0           # DW_AT_low_pc
	.long	.Lfunc_end1-.Lfunc_begin0 # DW_AT_high_pc
	.byte	2                       # Abbrev [2] 0x2a:0x1e DW_TAG_subprogram
	.quad	.Lfunc_begin0           # DW_AT_low_pc
	.long	.Lfunc_end0-.Lfunc_begin0 # DW_AT_high_pc
	.byte	1                       # DW_AT_frame_base
	.byte	87
	.long	.Linfo_string3          # DW_AT_linkage_name
	.long	.Linfo_string3          # DW_AT_name
	.byte	1                       # DW_AT_decl_file
	.byte	10                      # DW_AT_decl_line
	.long	102                     # DW_AT_type
	.byte	1                       # DW_AT_accessibility
                                        # DW_ACCESS_public
	.byte	2                       # Abbrev [2] 0x48:0x1e DW_TAG_subprogram
	.quad	.Lfunc_begin1           # DW_AT_low_pc
	.long	.Lfunc_end1-.Lfunc_begin1 # DW_AT_high_pc
	.byte	1                       # DW_AT_frame_base
	.byte	86
	.long	.Linfo_string5          # DW_AT_linkage_name
	.long	.Linfo_string6          # DW_AT_name
	.byte	1                       # DW_AT_decl_file
	.byte	20                      # DW_AT_decl_line
	.long	102                     # DW_AT_type
	.byte	1                       # DW_AT_accessibility
                                        # DW_ACCESS_public
	.byte	3                       # Abbrev [3] 0x66:0x7 DW_TAG_base_type
	.long	.Linfo_string4          # DW_AT_name
	.byte	0                       # DW_AT_encoding
	.byte	2                       # DW_AT_byte_size
	.byte	0                       # End Of Children Mark
.Ldebug_info_end0:
	.section	.debug_pubnames,"",@progbits
	.long	.LpubNames_end0-.LpubNames_begin0 # Length of Public Names Info
.LpubNames_begin0:
	.short	2                       # DWARF Version
	.long	.Lcu_begin0             # Offset of Compilation Unit Info
	.long	110                     # Compilation Unit Length
	.long	72                      # DIE offset
	.asciz	"main"                  # External Name
	.long	42                      # DIE offset
	.asciz	"s.func"                # External Name
	.long	0                       # End Mark
.LpubNames_end0:
	.section	.debug_pubtypes,"",@progbits
	.long	.LpubTypes_end0-.LpubTypes_begin0 # Length of Public Types Info
.LpubTypes_begin0:
	.short	2                       # DWARF Version
	.long	.Lcu_begin0             # Offset of Compilation Unit Info
	.long	110                     # Compilation Unit Length
	.long	102                     # DIE offset
	.asciz	"void"                  # External Name
	.long	0                       # End Mark
.LpubTypes_end0:
	.section	".note.GNU-stack","",@progbits
	.section	.debug_line,"",@progbits
.Lline_table_start0:
