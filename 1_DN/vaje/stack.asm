. import with: EXTREF sinit,spush,spop,sp
. exportamo samo to kar od zunaj uporabljamo!
. vse te spremenljivke, ki jih od zunaj uporabljamo, uporabljamo 4. format (ker nikoli nevemo koliko dalec so stran)
stk     EXTDEF sinit,spush,spop,sp

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

        END stk
