-- R を 199 回出力
"solve lambdaman6 " . (
  (
    (\f -> (\x -> f (x x)) (\x -> f (x x))) -- Yコンビネータ
    (\self -> \n ->
      if n == 0 then
        ""
      else
        (self (n - 1)) . "R"
    )
  ) 199
)
