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
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	subq	$96, %rsp
	.cfi_def_cfa_offset 128
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %rbp, -16
	.loc	1 20 0 prologue_end     # program.ke:20:0
	movl	$10, 48(%rsp)
	movl	48(%rsp), %eax
	movl	%eax, 40(%rsp)
	movl	40(%rsp), %eax
	movl	$0, 56(%rsp)
	movl	%eax, 32(%rsp)
	leaq	32(%rsp), %rax
	movq	%rax, 64(%rsp)
	movslq	56(%rsp), %rdi
	movq	vtables(,%rdi,8), %rax
	movq	64(%rsp), %rsi
                                        # kill: def $edi killed $edi killed $rdi
	movl	$2, %edx
	callq	*%rax
	movb	$-123, 11(%rsp)
	movb	$-90, 10(%rsp)
	movb	$-97, 9(%rsp)
	movb	$-16, 8(%rsp)
	movb	$32, 7(%rsp)
	movb	$108, 6(%rsp)
	movb	$101, 5(%rsp)
	movb	$114, 4(%rsp)
	movb	$116, 3(%rsp)
	movb	$115, 2(%rsp)
	movb	$101, 1(%rsp)
	movb	$75, (%rsp)
	movb	(%rsp), %sil
	movb	1(%rsp), %dil
	movb	2(%rsp), %r8b
	movb	3(%rsp), %r9b
	movb	4(%rsp), %r10b
	movb	5(%rsp), %r11b
	movb	6(%rsp), %bpl
	movb	7(%rsp), %r14b
	movb	8(%rsp), %dl
	movb	9(%rsp), %bl
	movb	10(%rsp), %al
	movb	11(%rsp), %cl
	movl	$1, 72(%rsp)
	movb	%cl, 27(%rsp)
	movb	%al, 26(%rsp)
	movb	%bl, 25(%rsp)
	movb	%dl, 24(%rsp)
	movb	%r14b, 23(%rsp)
	movb	%bpl, 22(%rsp)
	movb	%r11b, 21(%rsp)
	movb	%r10b, 20(%rsp)
	movb	%r9b, 19(%rsp)
	movb	%r8b, 18(%rsp)
	movb	%dil, 17(%rsp)
	movb	%sil, 16(%rsp)
	leaq	16(%rsp), %rax
	movq	%rax, 80(%rsp)
	leaq	72(%rsp), %rax
	movq	%rax, 88(%rsp)
	addq	$96, %rsp
	.cfi_def_cfa_offset 32
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Ltmp1:
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
	.byte	87
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
