let Y = \f -> (\x -> f (x x)) (\x -> f (x x)) in
let repeat = \s -> Y (\self -> \n -> if n == 0 then "" else (self (n - 1)) . s) in

let hilbert = Y
(
    \self -> \order -> \a -> \b -> \c -> \d -> (
        if order == 0 then
            ""
        else
            (self (order - 1) c d a b) .
            (repeat d 5) .
            (self (order - 1) a b c d) .
            (repeat b 5) .
            (self (order - 1) a b c d) .
            (repeat c 5) .
            (self (order - 1) d c b a)
    )
) in "solve lambdaman16 " . (hilbert 6 "L" "R" "U" "D")
