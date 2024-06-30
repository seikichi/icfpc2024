let Y = \f -> (\x -> f (x x)) (\x -> f (x x)) in
"solve lambdaman9 " . (
  (
    Y (\self -> \count ->
      if (count % 2500) == 0 then
        ""
      else
        if (count % 50) == 0 then
          "D" . (self (count + 1))
        else
          if (((count / 50) % 2) == 1) then
            "L" . (self (count + 1))
          else
            "R" . (self (count + 1))
    )
  )
  1
)

-- B$ L! B. S3/,6%},!-"$!-!.^} B$ B$ v! L$ L% ? B= B% v% I;Y I! S ? B= B% v% IS I! B. S> B$ v$ B+ v% I" ? B= B% B/ v% IS I# I" B. SF B$ v$ B+ v% I" B. SL B$ v$ B+ v% I" I" L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v#
