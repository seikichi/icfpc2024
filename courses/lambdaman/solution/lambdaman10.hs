"solve lambdaman10 " . (
  let f = \s -> s . s . s . s . s in
  let r = \s -> f (f s) in
  let rrR_DRRU = r (r "R" . "DRRU") in
  let rrL_ULLD = r (r "L" . "ULLD") in
  f (
    r (rrR_DRRU . "LD" . rrL_ULLD . "RD")
    . (rrR_DRRU . "DLD" . rrL_ULLD . "RDLU")
  )
)
