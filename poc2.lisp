// Proof of concept for the MLL format in its hierarchical form
(Y Year
    (A Album
        (S Song
            (A Artist                       ; in this case there may be multiple artists in this song, and the nested note scope is made by this artist
                (N                          ; note definition, overrides default note def (c, C, d, D...) with the first argument being the only wrapper node (notes expand right after)
                    (c name                 ; "channel" with a name from which singletons can be found
                        a b c               ; normal notes with default length and octave
                        (l 4 d e)           ; length with the number of notes in a beat (DIFFERENT from decimal rhythm representation, here odd numbers are tuplets)
                        (o 4 d e)           ; octave 
                        (x 2 d e)           ; loop two times (number in argument)
                        (L 10 0 10 0 d e)   ; legato with the sliding time at the beginning, static (0) in ms or percentage (1) of the note length, and the sliding time at the end with the same parametter.
                    )
                    c d e f g a b
                )
            )
        )
    )
)