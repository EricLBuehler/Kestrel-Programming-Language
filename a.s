	.text
	.file	"program.ke"
	.globl	_main                   # -- Begin function _main
	.p2align	4, 0x90
	.type	_main,@function
_main:                                  # @_main
.Lfunc_begin0:
	.file	1 "./program.ke"
	.loc	1 0 0                   # program.ke:0:0
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
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	pushq	%rax
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	jmp	.LBB0_1
.LBB0_1:                                # %loop_head
                                        # =>This Inner Loop Header: Depth=1
	jmp	.LBB0_2
.LBB0_2:                                # %loop_then
                                        #   in Loop: Header=BB0_1 Depth=1
	movq	%rsp, %rax
	movq	%rax, %rcx
	addq	$-16, %rcx
	movq	%rcx, %rsp
	movb	$10, -3(%rax)
	movb	$33, -4(%rax)
	movb	$100, -5(%rax)
	movb	$108, -6(%rax)
	movb	$114, -7(%rax)
	movb	$111, -8(%rax)
	movb	$119, -9(%rax)
	movb	$32, -10(%rax)
	movb	$44, -11(%rax)
	movb	$111, -12(%rax)
	movb	$108, -13(%rax)
	movb	$108, -14(%rax)
	movb	$101, -15(%rax)
	movb	$72, -16(%rax)
	movb	-16(%rax), %cl
	movb	%cl, -42(%rbp)          # 1-byte Spill
	movb	-15(%rax), %cl
	movb	%cl, -41(%rbp)          # 1-byte Spill
	movb	-14(%rax), %r9b
	movb	-13(%rax), %r10b
	movb	-12(%rax), %r11b
	movb	-11(%rax), %r14b
	movb	-10(%rax), %r15b
	movb	-9(%rax), %r12b
	movb	-8(%rax), %r13b
	movb	-7(%rax), %r8b
	movb	-6(%rax), %bl
	movb	-5(%rax), %sil
	movb	-4(%rax), %dl
	movb	-3(%rax), %cl
	movq	%rsp, %rdi
	movq	%rdi, %rax
	addq	$-16, %rax
	movq	%rax, %rsp
	movb	%cl, -3(%rdi)
	movb	%dl, -4(%rdi)
	movb	%sil, -5(%rdi)
	movb	%bl, -6(%rdi)
	movb	%r8b, -7(%rdi)
	movb	%r13b, -8(%rdi)
	movb	%r12b, -9(%rdi)
	movb	%r15b, -10(%rdi)
	movb	%r14b, -11(%rdi)
	movb	%r11b, -12(%rdi)
	movb	%r10b, -13(%rdi)
	movb	%r9b, -14(%rdi)
	movb	-41(%rbp), %cl          # 1-byte Reload
	movb	%cl, -15(%rdi)
	movb	-42(%rbp), %cl          # 1-byte Reload
	movb	%cl, -16(%rdi)
	movq	%rax, %rdi
	movb	$0, %al
	callq	printf
	jmp	.LBB0_1
.Ltmp1:
.Lfunc_end0:
	.size	_main, .Lfunc_end0-_main
	.cfi_endproc
                                        # -- End function
	.globl	main                    # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
.Lfunc_begin1:
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
                                        # kill: killed $rsi
                                        # kill: killed $edi
	callq	_main
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
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
	.byte	1                       # Abbrev [1] 0xb:0x43 DW_TAG_compile_unit
	.long	.Linfo_string0          # DW_AT_producer
	.short	2                       # DW_AT_language
	.long	.Linfo_string1          # DW_AT_name
	.long	.Lline_table_start0     # DW_AT_stmt_list
	.long	.Linfo_string2          # DW_AT_comp_dir
                                        # DW_AT_GNU_pubnames
	.quad	.Lfunc_begin0           # DW_AT_low_pc
	.long	.Lfunc_end0-.Lfunc_begin0 # DW_AT_high_pc
	.byte	2                       # Abbrev [2] 0x2a:0x1c DW_TAG_subprogram
	.quad	.Lfunc_begin0           # DW_AT_low_pc
	.long	.Lfunc_end0-.Lfunc_begin0 # DW_AT_high_pc
	.byte	1                       # DW_AT_frame_base
	.byte	86
	.long	.Linfo_string3          # DW_AT_linkage_name
	.long	.Linfo_string4          # DW_AT_name
	.long	70                      # DW_AT_type
	.byte	1                       # DW_AT_accessibility
                                        # DW_ACCESS_public
	.byte	3                       # Abbrev [3] 0x46:0x7 DW_TAG_base_type
	.long	.Linfo_string5          # DW_AT_name
	.byte	0                       # DW_AT_encoding
	.byte	2                       # DW_AT_byte_size
	.byte	0                       # End Of Children Mark
.Ldebug_info_end0:
	.section	.debug_pubnames,"",@progbits
	.long	.LpubNames_end0-.LpubNames_begin0 # Length of Public Names Info
.LpubNames_begin0:
	.short	2                       # DWARF Version
	.long	.Lcu_begin0             # Offset of Compilation Unit Info
	.long	78                      # Compilation Unit Length
	.long	42                      # DIE offset
	.asciz	"main"                  # External Name
	.long	0                       # End Mark
.LpubNames_end0:
	.section	.debug_pubtypes,"",@progbits
	.long	.LpubTypes_end0-.LpubTypes_begin0 # Length of Public Types Info
.LpubTypes_begin0:
	.short	2                       # DWARF Version
	.long	.Lcu_begin0             # Offset of Compilation Unit Info
	.long	78                      # Compilation Unit Length
	.long	70                      # DIE offset
	.asciz	"void"                  # External Name
	.long	0                       # End Mark
.LpubTypes_end0:
	.section	".note.GNU-stack","",@progbits
	.section	.debug_line,"",@progbits
.Lline_table_start0:
