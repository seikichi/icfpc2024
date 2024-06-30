-- rlpack.rb でパックした整数をもとの文字列に戻す
-- dir: 2bit (LRDU)
-- len: 6bit

-- Yコンビネータ
-- Y = λf . (λx . f (x x)) (λx . f (x x))
let Y = \f -> (\x -> f (x x)) (\x -> f (x x)) in

(
  Y (\self -> \code ->
    if code == 0 then
      ""
    else
      (self (code / 256)) . (
        (\s -> Y (\self -> \n -> if n == 0 then "" else (self (n - 1)) . s)) -- repeat(s, n)
        (1 T ((code % 4) D "LRDU")) -- LRDU
        ((code / 4) % 64) -- run length
      )
  )
)
4944161741967041562377 -- ←この数字を問題の解答を rlpack したものに書き換える 
