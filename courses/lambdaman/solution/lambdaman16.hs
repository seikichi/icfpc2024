"solve lambdaman16 " .
(\f -> f f) (
    \self -> \order -> \a -> \b -> \c -> \d -> (
        if order == 0 then
            ""
        else
            let next = self self (order - 1) in (
                (next c d a b) .
                d .
                (next a b c d) .
                b .
                (next a b c d) .
                c .
                (next d c b a)
            )
    )
) 6 "LLLL" "RRRR" "UUUU" "DDDD"
