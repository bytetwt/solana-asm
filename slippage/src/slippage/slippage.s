.equ ACCOUNT_BALANCE, 0x00a0
.equ MINIMUM_AMOUNT, 0x2918

.globl entrypoint
entrypoint:
  ldxdw r2, [r1+ACCOUNT_BALANCE]
  ldxdw r3, [r1+MINIMUM_AMOUNT]

  jle r3, r2, end
  lddw r0, 1

  exit

end:
  exit