-- packint.rb でパックした整数をもとの文字列に戻す
(
  (
    (\f -> f f) (\self -> \code ->
      if code == 1 then
        "solve lambdaman5 "
      else
        (self self (code / 4)) . (1 T ((code % 4) D "LRDU"))
    )
  )
  1085634933 -- ←この数字を問題の解答を packint したものに書き換える
)
