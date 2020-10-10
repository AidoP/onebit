.text
.global entry

entry:
	# Initialise stack
	la sp,RAM_START
	addi sp,sp,1024

	call read
	mv a2,a0
	# Keep return value in a0 and copy to a2
	la a1,RANDOM_CODE
	call strcmp
	beqz a0,2f
	la a0,_ok_msg
	j 3f
	2:
	mv a0,a2
	call print
	la a0,_err_msg
	3:
	call println
	j reset

reset:
	# System reset vector is address 0
	li a0,1
	sb a0,(zero)

read:
	# Save old stack
	mv t0,sp
	li t1,0x0A
	# Allocate 64B of stack
	addi sp,sp,-64
	1:
		# Load in a character
		lb a0,1(zero)
		# Break if the character is zero, newline or our 64B buffer is full
		beqz a0,1f
		beq a0,t1,1f
		ble t0,sp,1f
		# Save the character into the array on the stack
		sb a0,(sp)
		# Increment the stack
		addi sp,sp,1
		# Loop
		j 1b
	1:
	# Null terminate the string
	sb zero,(sp)
	# Return the char[64] array
	addi a0,t0,-64
	ret
print:
	1:
		# Load the character
		lb a1,(a0)
		# Exit if null
		beqz a1,1f
		# Print the character
		sb a1,2(zero)
		# Increment pointer
		addi a0,a0,1
		j 1b
	1:
	ret
println:
	# Save return address
	addi sp,sp,-4
	sw ra,(sp)

	call print
	# Print new line
	li a0,0x0A
	sb a0,2(zero)

	# Restore return address
	lw ra,(sp)
	addi sp,sp,4
	ret

strcmp:
	1:
		lb t0,(a0)
		lb t1,(a1)
		# Not equal
		bne t0,t1,1f
		# Strings are equal
		beqz t0,2f
		# Increment and loop
		addi a0,a0,1
		addi a1,a1,1
		j 1b
	1:
	# Strings are not equal; return 0
	addi a0,zero,0
	ret
	2:
	# Strings are equal; return 1
	addi a0,zero,1
	ret	


.rodata
_err_msg:
	.asciz " is an invalid code!"
_ok_msg:
	.asciz "Code accepted! Secret is: pecan{dad896bb27d8349a8258d8340f078281}"
