. expected output for file FA.dev with input:
. -------------------------
. 1
. 9
. 11
. 0
. -------------------------
. is output to stdout:
. -------------------------
. 
. 1
. 362880
. 6362368
.
. -------------------------

rec     START 0

        +JSUB sinit

recloop LDA #10     . 0x0a
        WD #1
        CLEAR X
        CLEAR A
        CLEAR S     . S == N

rdloop  RMO S, A    . S *= 10
        MUL #10
        RMO A, S

        CLEAR A     . get digit
        RD #250     . 0xfa
        IF #10 reccont
        . COMP #10    . 0x0a
        . JEQ reccont
        IF #0 halt
        . COMP #0
        . JEQ halt

        SUB #48     . =0x30 adtoi(digit): ascii digit to integer
        ADDR A, S   . S += digit
        J rdloop

reccont RMO S, A    . A == N == S/10
        DIV #10
        JSUB fakrec . get fac(N)

        RMO A, S    . S == fac(N) . print to stdout
        CLEAR A
        CLEAR X
        STCH output
recout  RMO S, A
        COMP #0
        JEQ recpr

        JSUB modul
        TIX #0
        ADD #48     . =0x30
        STCH output, X

        RMO S, A
        DIV #10
        RMO A, S
        J recout

recpr   CLEAR A
        LDCH output, X
        COMP #0
        JEQ recloop
        WD #1

        RMO X, A    . X--
        SUB #1
        RMO A, X
        J recpr

halt    J halt

output  RESB 100

. (A % 10) = A - (A / 10) * 10
. result in A
. modul   STA x
.         DIV #10
.         MUL #10
.         RMO A, B
.         LDA x
.         SUBR B, A
modul   MOD #10
        RSUB

quot    WORD 1
x       RESW 1
y       WORD 10

. returns N! in A
. param A == N
fakrec  STL @sp
        JSUB spush
        STB @sp
        JSUB spush
        
        COMP #2     . base case
        JLT fakend

                    . B = N & A = (N-1)!
        RMO A, B    . recursion
        SUB #1
        JSUB fakrec
        MULR B, A

fakend  JSUB spop
        LDB @sp
        JSUB spop
        LDL @sp
        RSUB

. stack
. --------------------------------------------
. USAGE:
. PUSHA
.       STA @sp
.       JSUB spush
. POPA
.       JSUB spop
.       LDA @sp

. inicializira stack - nastavi sp na zacetek sklada
sinit  STA saved_a
            LDA #stack
            STA sp
            LDA saved_a
            RSUB

. poveca sp za dolzino besede (3)
spush       STA saved_a
            LDA sp
            ADD #3
            STA sp
            LDA saved_a
            RSUB

. zmanjsa sp za dolzino besede (3)
spop        STA saved_a
            LDA sp
            SUB #3
            STA sp
            LDA saved_a
            RSUB

saved_a     WORD 0
sp          WORD 0
stack       RESW 1000
. --------------------------------------------

        END rec
