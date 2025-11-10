. a_0 + x(a_1 + x(a_2 + x(a_3 + ... + x(a_n-1 + x(a_n)) ... )))
horner  START 0

        LDCH tab, X
        TIX #0
        ADDR A, S       . S = a_n
loop    LDCH tab, X
        LDT x
        MULR T, S       . S *= x
        ADDR A, S       . S += a_i
        TIX #len
        JEQ end         . if X == 0 => end
        J loop
end     STS result

halt    J halt

. podatki
x       WORD 2
tab     BYTE 1
        BYTE 2
        BYTE 3
        BYTE 4
        BYTE 5
last    EQU *
len     EQU last - tab
result  RESW 1

        END horner
