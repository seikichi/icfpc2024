-- R を 199 回出力
"solve lambdaman6 " . (
  let f = \s -> s . s . s in
  f (f (f (f "RRR")))
)
