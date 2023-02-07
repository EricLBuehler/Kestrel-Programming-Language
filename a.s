	.text
	.file	"program.ke"
	.globl	f                       # -- Begin function f
	.p2align	4, 0x90
	.type	f,@function
f:                                      # @f
.Lfunc_begin0:
	.file	1 "./program.ke"
	.loc	1 0 0                   # program.ke:0:0
	.cfi_sections .debug_frame
	.cfi_startproc
# %bb.0:                                # %entry
	movl	$0, -16(%rsp)
	movl	$100, -12(%rsp)
	movl	-16(%rsp), %eax
	movl	-12(%rsp), %edx
	movq	-8(%rsp), %rcx
	retq
.Ltmp0:
.Lfunc_end0:
	.size	f, .Lfunc_end0-f
	.cfi_endproc
                                        # -- End function
	.globl	_main                   # -- Begin function _main
	.p2align	4, 0x90
	.type	_main,@function
_main:                                  # @_main
.Lfunc_begin1:
	.loc	1 4 0                   # program.ke:4:0
	.cfi_startproc
# %bb.0:                                # %entry
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
.Ltmp1:
	.loc	1 4 0 prologue_end      # program.ke:4:0
	callq	f
	movq	%rcx, 32(%rsp)
	movl	%edx, 28(%rsp)
	movl	%eax, 24(%rsp)
	movl	24(%rsp), %eax
	movl	$0, 8(%rsp)
	movl	$0, 12(%rsp)
	leaq	8(%rsp), %rcx
	addq	$4, %rcx
	cmpl	8(%rsp), %eax
	jne	.LBB1_2
# %bb.1:                                # %pattern_0
	jmp	.LBB1_3
.LBB1_2:                                # %default
	jmp	.LBB1_3
.LBB1_3:                                # %end
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
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
	.section	.debug_str,"MS",@progbits,1
.Linfo_string0:
	.asciz	"Kestrel"               # string offset=0
.Linfo_string1:
	.asciz	"program.ke"            # string offset=8
.Linfo_string2:
	.asciz	"."                     # string offset=19
.Linfo_string3:
	.asciz	"f"                     # string offset=21
.Linfo_string4:
	.asciz	"{ i32, i32, i64 }"     # string offset=23
.Linfo_string5:
	.asciz	"_main"                 # string offset=41
.Linfo_string6:
	.asciz	"main"                  # string offset=47
.Linfo_string7:
	.asciz	"void"                  # string offset=52
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
	.byte	73                      # DW_AT_type
	.byte	19                      # DW_FORM_ref4
	.byte	50                      # DW_AT_accessibility
	.byte	11                      # DW_FORM_data1
	.byte	0                       # EOM(1)
	.byte	0                       # EOM(2)
	.byte	3                       # Abbreviation Code
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
	.byte	4                       # Abbreviation Code
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
	.byte	1                       # Abbrev [1] 0xb:0x68 DW_TAG_compile_unit
	.long	.Linfo_string0          # DW_AT_producer
	.short	2                       # DW_AT_language
	.long	.Linfo_string1          # DW_AT_name
	.long	.Lline_table_start0     # DW_AT_stmt_list
	.long	.Linfo_string2          # DW_AT_comp_dir
                                        # DW_AT_GNU_pubnames
	.quad	.Lfunc_begin0           # DW_AT_low_pc
	.long	.Lfunc_end1-.Lfunc_begin0 # DW_AT_high_pc
	.byte	2                       # Abbrev [2] 0x2a:0x1c DW_TAG_subprogram
	.quad	.Lfunc_begin0           # DW_AT_low_pc
	.long	.Lfunc_end0-.Lfunc_begin0 # DW_AT_high_pc
	.byte	1                       # DW_AT_frame_base
	.byte	87
	.long	.Linfo_string3          # DW_AT_linkage_name
	.long	.Linfo_string3          # DW_AT_name
	.long	100                     # DW_AT_type
	.byte	1                       # DW_AT_accessibility
                                        # DW_ACCESS_public
	.byte	3                       # Abbrev [3] 0x46:0x1e DW_TAG_subprogram
	.quad	.Lfunc_begin1           # DW_AT_low_pc
	.long	.Lfunc_end1-.Lfunc_begin1 # DW_AT_high_pc
	.byte	1                       # DW_AT_frame_base
	.byte	87
	.long	.Linfo_string5          # DW_AT_linkage_name
	.long	.Linfo_string6          # DW_AT_name
	.byte	1                       # DW_AT_decl_file
	.byte	4                       # DW_AT_decl_line
	.long	107                     # DW_AT_type
	.byte	1                       # DW_AT_accessibility
                                        # DW_ACCESS_public
	.byte	4                       # Abbrev [4] 0x64:0x7 DW_TAG_base_type
	.long	.Linfo_string4          # DW_AT_name
	.byte	0                       # DW_AT_encoding
	.byte	2                       # DW_AT_byte_size
	.byte	4                       # Abbrev [4] 0x6b:0x7 DW_TAG_base_type
	.long	.Linfo_string7          # DW_AT_name
	.byte	0                       # DW_AT_encoding
	.byte	2                       # DW_AT_byte_size
	.byte	0                       # End Of Children Mark
.Ldebug_info_end0:
	.section	.debug_pubnames,"",@progbits
	.long	.LpubNames_end0-.LpubNames_begin0 # Length of Public Names Info
.LpubNames_begin0:
	.short	2                       # DWARF Version
	.long	.Lcu_begin0             # Offset of Compilation Unit Info
	.long	115                     # Compilation Unit Length
	.long	70                      # DIE offset
	.asciz	"main"                  # External Name
	.long	42                      # DIE offset
	.asciz	"f"                     # External Name
	.long	0                       # End Mark
.LpubNames_end0:
	.section	.debug_pubtypes,"",@progbits
	.long	.LpubTypes_end0-.LpubTypes_begin0 # Length of Public Types Info
.LpubTypes_begin0:
	.short	2                       # DWARF Version
	.long	.Lcu_begin0             # Offset of Compilation Unit Info
	.long	115                     # Compilation Unit Length
	.long	107                     # DIE offset
	.asciz	"void"                  # External Name
	.long	100                     # DIE offset
	.asciz	"{ i32, i32, i64 }"     # External Name
	.long	0                       # End Mark
.LpubTypes_end0:
	.section	".note.GNU-stack","",@progbits
	.section	.debug_line,"",@progbits
.Lline_table_start0:
