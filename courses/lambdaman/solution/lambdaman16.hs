let repeat5 = \s -> s . s . s . s . s in

"solve lambdaman16 " .
(\f -> f f) (
    \self -> \order -> \a -> \b -> \c -> \d -> (
        if order == 0 then
            ""
        else
            (self self (order - 1) c d a b) .
            (repeat5 d) .
            (self self (order - 1) a b c d) .
            (repeat5 b) .
            (self self (order - 1) a b c d) .
            (repeat5 c) .
            (self self (order - 1) d c b a)
    )
) 6 "L" "R" "U" "D"
