. look below for expected result for variables

arith   START   0

        LDA     x
        ADD     y
        STA     sum

        LDA     x
        SUB     y     
        STA     diff

        LDA     x
        MUL     y
        STA     prod

        LDA     x
        DIV     y
        STA     quot

. (A % B) = A - (A / B) * B
        LDA     y
        MUL     quot
        STA     vmes
        LDA     x
        SUB     vmes
        STA     mod

halt    J       halt

. podatki
x       WORD    420
y       WORD    66

. rezultati
sum     RESW    1   . 1e6
diff    RESW    1   . 162
prod    RESW    1   . 6c48
quot    RESW    1   . 6

mod     RESW    1   . 18
vmes    RESW    1

        END arith
