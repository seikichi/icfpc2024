-- packint.rb でパックした整数をもとの文字列に戻す
-- 全体を左右反転したものを末尾に追加して unpack する
let unpack = \lrdu -> (
  (\f -> f f) (\self -> \code ->
    if code == 1 then
      ""
    else
      (self self (code / 4)) . (1 T ((code % 4) D lrdu))
  )
) in
(\code -> "solve lambdaman7 " . unpack "LRDU" code . unpack "RLDU" code)
231274505686763608377365567704000568967834755681028472936860017809262483982057432279573521730501120973313216030875855530863755936 -- ←この数字を問題の解答を packint したものに書き換える
