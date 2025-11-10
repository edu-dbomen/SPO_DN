poly    START 0

loop    LDCH tab, X
        JSUB potX
        ADDR A, S
        TIX #len
        JEQ end
        J loop
end     STS result

halt    J halt

. A *= x^X
potX    STX savedX  . shranimo staro vrednost X
loop2   LDT #0
        COMPR X, T
        JEQ end2    . ce X == 0 => end
        MUL x
        LDT #1
        SUBR T, X   . X--
        J loop2
end2    LDX savedX  . X nazaj na prvotno vrednost
        RSUB

. podatki
x       WORD 2
tab     BYTE 5
        BYTE 4
        BYTE 3
        BYTE 2
        BYTE 1
last    EQU *
len     EQU last - tab
savedX  RESW 1
result  RESW 1

        END poly
