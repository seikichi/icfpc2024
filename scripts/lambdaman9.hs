"solve lambdaman9 " . (
  (
    (\f -> (\x -> f (x x)) (\x -> f (x x))) (\self -> \count ->
      if count == 2500 then
        ""
      else
        (
          if (count % 50) == 0 then
            "D"
          else
            if (((count / 50) % 2) == 1) then
              "L"
            else
              "R"
        ) . (self (count + 1))
    )
  )
  1
)

-- B. S3/,6%},!-"$!-!.^} B$ B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L$ L% ? B= v% I;Y S B. ? B= B% v% IS I! S> ? B= B% B/ v% IS I# I" SF SL B$ v$ B+ v% I" I"
-- NOTE: Your score: 159. Best score: 110.
