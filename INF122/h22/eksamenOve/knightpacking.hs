main = do
  number <- getLine
  printer (read number)

printer :: Int -> IO ()
printer n
  | even n = putStrLn "second"
  | otherwise = putStrLn "first"

