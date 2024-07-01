-- Yコンビネータ
-- Y = λf . (λx . f (x x)) (λx . f (x x))
let Y = \f -> (\x -> f (x x)) (\x -> f (x x)) in

-- repeat s n: 文字列sをn回繰り返した文字列を返す
let repeat = \s -> Y (\self -> \n -> if n == 0 then "" else (self (n - 1)) . s) in

-- 2^n
let pow2 = Y (\self -> \n -> if n == 0 then 1 else 2 * self (n - 1)) in

"solve lambdaman19 " . (
    let t_sq = Y(
        \self -> \order -> \a -> \b -> \c -> \d -> (
            if order < 0 then
                ""
            else
                let next = self (order - 1) in (
                    let len = pow2 order in
                    (repeat a len) .
                    (next a b c d) .
                    (repeat c len) . (repeat b len) .
                    (next b c d a) .
                    (repeat d len) . (repeat d len) .
                    (next d a b c) .
                    (repeat b len)
                )
        )
    ) in
    (t_sq 6 "U" "R" "D" "L") . (t_sq 6 "D" "R" "U" "L")
)

