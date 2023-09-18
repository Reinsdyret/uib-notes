main = do 
  l <- getLine
  let [a,b] = words l
  putStrLn $ if (a > b) then "1" else "0"
