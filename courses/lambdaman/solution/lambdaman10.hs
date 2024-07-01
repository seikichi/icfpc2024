"solve lambdaman10 " . (
  let f = \s -> s . s . s . s . s in
  let r = \s -> f (f s) in
  f (
    r (r (r "R" . "DRRU") . "LDR" . r (r "L" . "ULLD") . "RDL")
    . (r (r "R" . "DRRU") . "DLDR" . r (r "L" . "ULLD") . "RDLU")
  )
)
