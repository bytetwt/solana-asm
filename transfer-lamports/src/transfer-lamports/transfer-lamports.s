.equ ACCOUNT1_HEADER,   0x0008
.equ ACCOUNT2_HEADER,   0x2868
.equ ACCOUNT1_LAMPORTS, 0x0050
.equ ACCOUNT2_LAMPORTS, 0x28b0
.equ INSTRUCTION_DATA,  0x50d0

.globl entrypoint
entrypoint:
  ldxb  r2, [r1+ACCOUNT1_HEADER+1]  #; is signer
  jne   r2, 1, end

  ldxb  r2, [r1+ACCOUNT1_HEADER+2]  #; is writable
  jne   r2, 1, end

  ldxb  r2, [r1+ACCOUNT2_HEADER+2]  #; is writable
  jne   r2, 1, end

  ldxdw r2, [r1+ACCOUNT1_LAMPORTS]
  ldxdw r3, [r1+ACCOUNT2_LAMPORTS]
  ldxdw r4, [r1+INSTRUCTION_DATA]

  jlt   r2, r4, end

  sub64 r2, r4
  add64 r3, r4

  stxdw [r1+ACCOUNT1_LAMPORTS], r2
  stxdw [r1+ACCOUNT2_LAMPORTS], r3

  mov64 r0, 0

  exit

end:
  mov64 r0, 1
  exit
