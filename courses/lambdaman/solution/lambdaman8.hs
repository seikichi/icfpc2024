-- ぐるぐる回る
-- ↓←↑→ の方向に順番に歩く
-- 2歩から始まり、二回曲がるごとに歩く歩数が2歩ずつ増える

"solve lambdaman8 " .  (
  -- sを3回繰り返す
  let f = \s -> s . s . s in
  -- sを243回繰り返す
  let g = \s -> f (f (f (f (f s)))) in
  g (g "DL" . g "UR")
)
