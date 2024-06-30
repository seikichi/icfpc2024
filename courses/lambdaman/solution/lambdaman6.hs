-- R を 199 回出力
"solve lambdaman6 " . (
  let f = \s -> s . s . s in
  44 D (f (f (f (f "RRR"))))
)
