"solve lambdaman21 " . (
  let repeat = \s -> (\f -> f f) (\self -> \n -> if n == 0 then "" else (self self (n - 1)) . s) in
  let rU = repeat "U" in
  let rR = repeat "R" in
  let rD = repeat "D" in
  let rL = repeat "L" in
  (
    -- 一旦左上
    (rU 71) . (rL 76) .
    -- 塗れるだけ塗る
    (repeat ((rR 199) . "D" . (rL 199)) 199) .
    -- まずは 3 の左の下に
    (rU 50) .
    -- 内側になるまで塗る
    (repeat ((rR 100) . (rL 100) . "U") 105) .
    -- 3の塗り残し (上)
    (rD 17) .
    (rR 18) .
    "U" . (repeat ((rR 40) . (rL 10) . "U" . (rL 30)) 5) .
    -- 3の塗り残し (下)
    (rD 10) . (rL 21) . (rD 66) .
    (repeat ((rR 50) . (rL 10) . "D" . (rL 40)) 7) .
    -- 3D の間
    (rU 7) . (rL 22) . (rD 19) . (rR 91) . (rU 3) .
    (repeat ((rL 50) . (rR 50) . "U") 101) .
    -- D にいく
    (rR 86) . (rD 49) . (rL 15) . "U" .
    -- D ぬる
    (repeat ((rD 80) . "L" . (rU 80)) 56)
  )
)
