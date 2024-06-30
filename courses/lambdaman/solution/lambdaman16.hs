"solve lambdaman16 " .
(\f -> f f) (
    \self -> \order -> \a -> \b -> \c -> \d -> (
        if order == 0 then
            ""
        else
            let next = order - 1 in (
              (self self next c d a b) .
              d .
              (self self next a b c d) .
              b .
              (self self next a b c d) .
              c .
              (self self next d c b a)
            )
    )
) 6 "LLLL" "RRRR" "UUUU" "DDDD"
