"solve lambdaman9 " . (
  (
    (\f -> f f) (\self -> \count ->
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
        ) . (self self (count + 1))
    )
  )
  1
)

-- B. S3/,6%},!-"$!-!.^} B$ B$ L" B$ v" v" L$ L% ? B= v% I;Y S B. ? B= B% v% IS I! S> ? B= B% B/ v% IS I# I" SF SL B$ B$ v$ v$ B+ v% I" I"
-- NOTE: Your score: 135. Best score: 110.
