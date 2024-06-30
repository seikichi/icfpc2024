"solve lambdaman9 " . (
  -- sを3回繰り返す
  let f = \s -> s . s . s in
  -- sを81回繰り返す
  let g = \s -> f (f (f (f s))) in
  g (g "R" . g "L" . "D")
)
