main = getLine >>= printSumOf . read

printSumOf :: Int -> IO ()
printSumOf n = do
  line <- getLine
  let numList = map read (words line)
  putStrLn $ show $ sum $ map negate $ filter (<0) numList


