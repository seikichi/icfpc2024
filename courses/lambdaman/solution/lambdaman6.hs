-- R を 199 回出力
"solve lambdaman6 " . (
  (\f -> f (f (f "RRRRRRRR"))) (\s -> s . s . s)
)
