. main
. -------------------------------------------------------
scrn    START 0

        . set scrlen
        LDA scrcols
        MUL scrrows
        STA scrlen

.         LDA #0x41
.         JSUB scrfill
.         JSUB scrclear

        . read user input forever (keyboard)
read    CLEAR A
        RD #0
        COMP #0x0a  . if '\n' set cursor to new line
        JEQ set
        JSUB printch
        J read

set     JSUB crsr2newln
        J read

halt    J halt
. -------------------------------------------------------


. clear screen
. -------------------------------------------------------
scrclear    STL sc_retaddr  . no stack => have to save L

            LDA #0x00
            JSUB scrfill

            LDL sc_retaddr
            RSUB
. -------------------------------------------------------
. fill screen with byte in A
. -------------------------------------------------------
scrfill CLEAR T         . T == len
        LDT scrlen

        CLEAR S         . S == char of A
        ADDR A, S

        CLEAR X         . X == index

loop    COMPR X, T      . if EOF => return
        JEQ srcfill_ret

        LDA screen      . deref current address
        ADDR X, A
        STA sf_deref

        CLEAR A         . print to screen
        ADDR S, A
        STCH @sf_deref

        TIX #0          . X++
        J loop

srcfill_ret RSUB
. -------------------------------------------------------


. print char to screen
. -------------------------------------------------------
printch STCH @cursor    . print to cursor

        LDA cursor      . cursor++
        ADD #1
        STA cursor

        RSUB
. -------------------------------------------------------

. cursor to new line
. -------------------------------------------------------
crsr2newln  LDA cursor
            SUB screen
            DIV #80
            ADD #1
            MUL #80
            ADD screen
            STA cursor

            RSUB
. -------------------------------------------------------


. VARS
. -------------------------------------------------------
screen  WORD 0xb800
scrcols WORD 80
scrrows WORD 25
scrlen  RESW 1

sc_retaddr  RESW 1

sf_deref    RESW 1

cursor  WORD 0xb800
. -------------------------------------------------------

        END scrn
