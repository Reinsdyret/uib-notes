main = do
  line1 <- getLine
  line2 <- getLine
  putStrLn $ show $ getCombinations line1 line2

getCombinations :: String -> String -> Int
getCombinations l1 l2 = case ((countDistinctNums l1 l2) > 0) of
                          True  -> 5 + countDistinctNums l1 l2
                          False -> 1

countDistinctNums :: String -> String -> Int
countDistinctNums [] []         = 0
countDistinctNums (x:xs) (y:ys) = if x /= y then 1 + countDistinctNums xs ys else countDistinctNums xs ys

