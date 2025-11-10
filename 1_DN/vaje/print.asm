print   START 0

        CLEAR X
loop    LDCH txt, X
        WD #0xAA
        TIX #txtlen
        JLT loop

halt    J halt

txt     BYTE C'SIC/XE'
        BYTE 0x0a
txtend  EQU *
txtlen  EQU txtend - txt

        END print
