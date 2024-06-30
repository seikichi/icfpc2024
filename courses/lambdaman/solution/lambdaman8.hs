-- ぐるぐる回る
-- ↓←↑→ の方向に順番に歩く
-- 2歩から始まり、二回曲がるごとに歩く歩数が2歩ずつ増える

-- Yコンビネータ
-- Y = λf . (λx . f (x x)) (λx . f (x x))
let Y = \f -> (\x -> f (x x)) (\x -> f (x x)) in

-- repeat(s, n)
let repeat = \s -> Y (\self -> \n -> if n == 0 then "" else (self (n - 1)) . s) in

"solve lambdaman8 " .
(
  Y (\self -> \i ->
    if i == 97 then
      repeat "L" 97
    else
      let dir = 1 T ((i % 4) D "DLUR") in
      let streak = i / 2 * 2 + 2 in
      (repeat dir streak) . (self (i + 1))
  )
) 0
