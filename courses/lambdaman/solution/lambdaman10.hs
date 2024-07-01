"solve lambdaman10 " .
( let h =
  (\f -> f f) (
    \self -> \order -> \a -> \b -> \c -> \d -> (
        if order == 0 then
            ""
        else
            let next = self self (order - 1) in
            let nextabcd = (next a b c d) in (
                (next c d a b) .
                d .
                nextabcd .
                b .
                nextabcd .
                c .
                (next d c b a)
            )
    )
  ) in
  (h 7 "L" "R" "U" "D") . (h 7 "LL" "RR" "UU" "DD")
)
