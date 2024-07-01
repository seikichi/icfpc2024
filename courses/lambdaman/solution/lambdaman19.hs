-- Yコンビネータ
-- Y = λf . (λx . f (x x)) (λx . f (x x))
let Y = \f -> (\x -> f (x x)) (\x -> f (x x)) in

"solve lambdaman19 " . (
    let t_sq = Y(
        \self -> \order -> \a -> \b -> \c -> \d -> (
            if order < 0 then
                ""
            else
                let next = self (order - 1) in (
                    let repeat2n =
                        -- repeat
                        (Y (\self -> \n -> \s -> if n == 0 then "" else (self (n - 1) s) . s))
                        (
                            -- pow2
                            (Y (\self -> \n -> if n == 0 then 1 else 2 * self (n - 1)))
                            order
                        )
                    in
                    (repeat2n a) .
                    (next a b c d) .
                    (repeat2n c) . (repeat2n b) .
                    (next b c d a) .
                    (repeat2n d) . (repeat2n d) .
                    (next d a b c) .
                    (repeat2n b)
                )
        )
    ) in
    (t_sq 6 "U" "R" "D" "L") . (t_sq 6 "D" "R" "U" "L")
)

