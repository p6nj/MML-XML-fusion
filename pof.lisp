// Proof of concept for the MLL format
// ;title;album;artist;year;number of notes;comment
;Song;;Artist;2023;12;This is a test song. You can put a semicolon there ; it's the last field after all. The first character of the first real line define the separator.
(
    a b c               ; normal notes with default length and octave
    (l 4 d e)           ; length with the number of notes in a beat (DIFFERENT from decimal rhythm representation, here odd numbers are tuplets)
    (o 4 d e)           ; octave 
    (x 2 d e)           ; loop two times (number in argument)
    (L 10 0 10 0 d e)   ; legato with the sliding time at the beginning, static (0) in ms or percentage (1) of the note length, and the sliding time at the end with the same parametter.
)