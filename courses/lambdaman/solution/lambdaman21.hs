"solve lambdaman21 " . (
  let repeat = \s -> (\f -> f f) (\self -> \n -> if n == 0 then "" else (self self (n - 1)) . s) in
  let rU = repeat "U" in
  let rR = repeat "R" in
  let rD = repeat "D" in
  let rL = repeat "L" in
  let rU200 = rU 200 in
  -- let rD200 = rD 200 in
  let rR200 = rR 200 in
  let rL200 = rL 200 in
  let rL10 = rL 10 in
  (
    -- 一旦左上 (U, L 適当)
    rU200 . rL200 .
    -- 塗れるだけ塗る (199 より大きくても良い)
    (repeat (rR200 . "D" . rL200) 199) .
    -- まずは 3 の左の下に
    (rU 50) .
    -- 内側になるまで塗る (rR と RL 適当でいい)
    (repeat (rR200 . rL200 . "U") 105) .
    -- 3の塗り残し (上)
    (rD 17) .
    (rR 18) .
    "U" . (repeat (rR200 . rL10 . "U" . rL200) 5) .
    -- 3の塗り残し (下)
    (rD 10) . (rL 21) . (rD 66) .
    (repeat (rR200 . rL10 . "D" . rL200) 7) .
    -- 3D の間
    (rU 7) . (rL 22) . (rD 19) . (rR 91) . (rU 3) .
    (repeat (rL200 . rR200 . "U") 101) .
    -- D にいく
    (rR 86) . (rD 49) . (rL 15) . "U" .
    -- D ぬる
    (repeat ((rD 80) . "L" . rU200) 56)
  )
)
