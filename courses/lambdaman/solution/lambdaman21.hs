"solve lambdaman21 " . (
  let Y = \f -> (\x -> f (x x)) (\x -> f (x x)) in
  let repeat = \s -> Y (\self -> \n -> if n == 0 then "" else (self (n - 1)) . s) in
  (
    -- 一旦左上
    (repeat "U" 71) . (repeat "L" 76) .
    -- 塗れるだけ塗る
    (repeat ((repeat "R" 199) . "D" . (repeat "L" 199)) 199) .
    -- まずは 3 の左の下に
    (repeat "U" 50) .
    -- 内側になるまで塗る
    (repeat ((repeat "R" 100) . (repeat "L" 100) . "U") 105) .
    -- 3の塗り残し (上)
    (repeat "D" 17) .
    (repeat "R" 18) .
    "U" . (repeat ((repeat "R" 40) . (repeat "L" 10) . "U" . (repeat "L" 30)) 5) .
    -- 3の塗り残し (下)
    (repeat "D" 10) . (repeat "L" 21) . (repeat "D" 66) .
    (repeat ((repeat "R" 50) . (repeat "L" 10) . "D" . (repeat "L" 40)) 7) .
    -- 3D の間
    (repeat "U" 7) . (repeat "L" 22) . (repeat "D" 19) . (repeat "R" 91) . (repeat "U" 3) .
    (repeat ((repeat "L" 50) . (repeat "R" 50) . "U") 101) .
    -- D にいく
    (repeat "R" 86) . (repeat "D" 49) . (repeat "L" 15) . "U" .
    -- D ぬる
    (repeat ((repeat "D" 80) . "L" . (repeat "U" 80)) 56)
  )
)
