-- R を 199 回出力
"solve lambdaman6 " . (
  let dbl = \s -> s . s in
  57 D (dbl (dbl (dbl (dbl (dbl "RRRRRRRR")))))
)
