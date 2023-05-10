; Proof of concept for the MLL format in its hierarchical form, digits for notes
; Notes names are strings, which means they can contain multiple characters and it won't alter parsing performance of the interface-side text as every chunk of characters (if recognized as a note) will have its own note linked using the text index.
; All numbers are 2 digits hex by default except in the instrument definition and tempi. Decimal numbers are prefixed with '.
(Y Year
    (A Album
        (S Song
            (a Artist                                       ; in this case there may be multiple artists in this song, and the nested note scope is made by this artist
                (T 120                                      ; tempo (BPM)
                    (N                                      ; note definition, overrides default note def (c, C, d, D...) with the first argument being the only wrapper node (notes expand right after)
                        (I                                  ; instruments, overrides default sine @442 with three different sound waves in tuples (sine `i`, square `q` and triangle `t`) which can be combined with 2 operators and with frequencies defined statically or relationally with other operators.
                            (C name                         ; "channel" with a name from which singletons can be found
                                0 1 2                       ; normal notes with default length and octave, code is the index of the note in the note list of the note def wrapper in hex up to 2 digits.
                                (l 4 1 2)                   ; length with the number of notes in a beat (DIFFERENT from decimal rhythm representation, here odd numbers are tuplets) in hex up to 2 digits.
                                (o 4 1 2)                   ; octave in hex up to 2 digits.
                                (x 2 1 2)                   ; loop two times (hex number in argument)
                                (G '10 0 1 2)               ; glissando with the sliding time at the beginning, static (decimal numbers indicated by ') in ms or percentage (2 digit hex) of the note length, and the sliding time at the end with the same parameter.
                                (V 1 1 '10 f0 1 2)          ; vibrato with the amplitude (hex), frequency (hex), and the other repartition parameters defined above for legato.
                                (v pp 1 2)                  ; volume, overrides default mf (MezzoForte, see enum) with dynamics written just like on sheet music.
                                (F pp f 1 1 2)              ; fader, applies a volume fade on the inner notes (while they are being played with 0 or between notes with 1 as shown here).
                                (~ '1 ff 0 ff ff ff 1 2)    ; ADS-R with for each component a length (static or dynamic in hex that divides the length left by static) and a final volume in hex (percentage of the max available volume). Only the first component (attack) is necessary, the default is '0 ff 0 ff 0 ff ff (nothing but max level sustain). NOTE: the release part is ommitted as same channel note overlap isn't to be implemented.
                                (M 1 0 0 1 0 -1 0)          ; mask definition: mask type (0 for rhythm, 1 for note), mask id (hex) and signed hex for the layer (here we have the implementation of a turn, some kind of tremolo for a single note).
                                (m 0 1)                     ; masked note: mask id (hex) and bare notes or restricted wrappers (the 1 here will be turned into 1 2 1 0 1 fitting the single note length). See corresponding enums for more info.
                                (t 120 130 1 2)             ; accelerando / deceleration, tempo shift from the first param to the next param (here accelerating from 120bpm to 130bpm). After it the tempo is still defined by the parent clause.
                            )
                            i 442 * q *2 + t +0
                        )
                        c d e f g a b
                    )
                )
            )
        )
    )
)
; Todo: finish this poc with new enum items