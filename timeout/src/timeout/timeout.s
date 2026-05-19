.equ CLOCK_ACCOUNT_DATA, 0x0060
.equ INSTRUCTION_DATA, 0x0098

.globl entrypoint
entrypoint:
  ldxdw r2, [r1+CLOCK_ACCOUNT_DATA]
  ldxdw r3, [r1+INSTRUCTION_DATA]

  jle r2, r3, end
  mov64 r0, 1

  exit

end:
  exit
