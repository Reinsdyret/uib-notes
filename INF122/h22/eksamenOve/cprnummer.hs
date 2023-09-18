main = getLine >>= \nums ->
  if mod (sumCpr nums) 11 == 0 then putStrLn "1" else putStrLn "0"
  
sumCpr :: String -> Int
sumCpr str = sumCprLeft (take 6 str) + sumCprRight (drop 7 str)

rtd :: Int -> String -> Int
rtd d c = read $ take 1 $ drop d c

sumCprLeft :: String -> Int
sumCprLeft num = 
  ((read $ take 1 num) * 4) + 
  ((rtd 1 num) * 3) + 
  ((rtd 2 num) * 2) + 
  ((rtd 3 num) * 7) + 
  ((rtd 4 num) * 6) + 
  ((rtd 5 num) * 5)

sumCprRight :: String -> Int
sumCprRight num = (read $ take 1 num) * 4 + 
  (rtd 1 num) * 3 +
  (rtd 2 num) * 2 +
  (rtd 3 num) * 1

