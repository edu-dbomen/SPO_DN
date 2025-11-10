. NOTE: num code in C at the bottom


. main
. -------------------------------------------------------
print   START 0

        LDCH #0x41
        JSUB nl
        JSUB char
        JSUB nl
        LDA #myStr
        JSUB string
        JSUB nl
        LDA myNum
        JSUB num
        JSUB nl

halt    J halt
. -------------------------------------------------------


. print char in A to stdout
. -------------------------------------------------------
char    WD #1
        RSUB
. -------------------------------------------------------


. print '\n' to stdout
. -------------------------------------------------------
nl      STA savedA
        CLEAR A
        LDCH #0x0a
        WD #1
        LDA savedA
        RSUB
. -------------------------------------------------------


. print string in (A) to stdout
. -------------------------------------------------------
string  STA savedA

        STA var1
        CLEAR A
loop    LDCH @var1
        COMP #0x00  . if EOF => ret
        JEQ ret
        WD #1

. var1++ (point to the addr of the next char)
        LDA var1
        ADD #1
        STA var1
        J loop

ret     LDA savedA
        RSUB
. -------------------------------------------------------


. print num in (A) to stdout
. signed positive numbers only!
. -------------------------------------------------------
num     STA savedA
        CLEAR S
        ADDR L ,S   . save ret addr to S, since we nest

        STA number
        COMP #0
        JEQ edge    . edge case
        CLEAR X
        CLEAR A

        STCH tab    . tab[0] = 0

. first while loop - getting digits (look at C code below)
loop1   TIX #0
        LDA number
        COMP #0
        JEQ endl1
        JSUB mod10
        ADD #0x30   . 0x30 == '0'
        STCH tab, X
        LDA number
        DIV #10
        STA number
        J loop1

endl1   CLEAR A
        JSUB xmin1
. second while loop - reading digits from behind (look at C code below)
loop2   LDCH tab, X
        COMP #0
        JEQ numret
        JSUB char
        JSUB xmin1
        J loop2

numret  LDA savedA
        CLEAR L
        ADDR S, L   . use saved ret addr
        RSUB

edge    ADD #0x30
        WD #1
        J numret

xmin1   LDT #1
        SUBR T, X
        RSUB
. -------------------------------------------------------


. VARS
. -------------------------------------------------------
savedA  RESW 1
myStr   BYTE C'Hello World'
        BYTE 0x00

. var for string subroutine for storing A value
var1    RESW 1

. vars for num subroutine
myNum   WORD 1234567
number  RESW 1
tab     RESB 9
. -------------------------------------------------------


. reused from arith.asm
. -------------------------------------------------------
. (A % B) = A - (A / B) * B
. result in A
mod10   STA     _a
        DIV     #10
        MUL     #10
        STA     vmes
        LDA     _a
        SUB     vmes
        RSUB

_a      RESW    1
vmes    RESW    1
. -------------------------------------------------------

        END print

. num C code
. -------------------------------------------------------
. #include <stdio.h>
. 
. int main() {
. 
.     int num = 1234567;
.     char tab[8];   // 24b registri => max signed 8388607 => 7 digits => + 0x00
.                    // for EON => 8 digits
.     tab[0] = 0x00; // tab[0] == EON
.     int x = 0;
. 
.     if (num == 0) {
.         printf("%c", '0'); // edge case
.         return 0;
.     }
. 
.     while (num > 0) {
.         x++;
.         tab[x] = (num % 10) + '0';
.         num /= 10;
.     }
. 
.     while (tab[x] != 0) {
.         printf("%c", tab[x]);
.         x--;
.     }
. 
.     return 0;
. }
. -------------------------------------------------------
