-- packint.rb でパックした整数をもとの文字列に戻す
1 D ((
  (\x ->
    (\y -> x (y y)) (\y -> x (y y))
  ) (\self -> \code ->
    if code == 0 then
      ""
    else
      (self (code / 4)) . (1 T ((code % 4) D "LRDU"))
  )
)
1085634933 -- ←この数字を問題の解答を packint したものに書き換える
)

