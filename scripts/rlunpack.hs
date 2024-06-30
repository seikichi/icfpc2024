-- rlpack.rb でパックした整数をもとの文字列に戻す
-- dir: 2bit (LRDU)
-- len: 6bit

(
  (\f -> f f) (\self -> \code ->
    if code == 0 then
      "solve lambdaman4 "
    else
      (self self (code / 256)) . (
        -- repeat(s, n)
        (\s -> (\f -> f f) (\self -> \n -> if n == 0 then "" else (self self (n - 1)) . s))
        -- LRDU
        (1 T ((code % 4) D "LRDU"))
        -- run length
        ((code / 4) % 64)
      )
  )
)
4944161741967041562377 -- ←この数字を問題の解答を rlpack したものに書き換える 
