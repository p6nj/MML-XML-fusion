// Proof of concept for the MLL format in its hierarchical form, digits for notes
// Notes names are strings, which means they can contain multiple characters and it won't alter parsing performance of the interface-side text as every chunk of characters (if recognized as a note) will have its own note linked using the text index.
(Y Year
    (A Album
        (S Song
            (A Artist                       ; in this case there may be multiple artists in this song, and the nested note scope is made by this artist
                (N                          ; note definition, overrides default note def (c, C, d, D...) with the first argument being the only wrapper node (notes expand right after)
                    (C name                 ; "channel" with a name from which singletons can be found
                        0 1 2               ; normal notes with default length and octave, code is the indice of the note in the note list of the notedef wrapper
                        (l 4 1 2)           ; length with the number of notes in a beat (DIFFERENT from decimal rhythm representation, here odd numbers are tuplets)
                        (o 4 1 2)           ; octave 
                        (x 2 1 2)           ; loop two times (number in argument)
                        (L 10 0 10 0 1 2)   ; legato with the sliding time at the beginning, static (0) in ms or percentage (1) of the note length, and the sliding time at the end with the same parametter.
                    )
                    c d e f g a b
                )
            )
        )
    )
)