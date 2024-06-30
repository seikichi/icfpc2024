-- R を 199 回出力
-- Yコンビネータ
-- Y = λf . (λx . f (x x)) (λx . f (x x))
let Y = \f -> (\x -> f (x x)) (\x -> f (x x)) in

"solve lambdaman6 " . (
  (
    Y (\self -> \n ->
      if n == 0 then
        ""
      else
        (self (n - 1)) . "R"
    )
  ) 199
)
