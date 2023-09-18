main = do
  line <- getLine
  if hissa line then putStrLn "hiss" else putStrLn "no hiss"

-- True if at any point in the string there are two consecutive s's
hissa :: String -> Bool
hissa []        = False
hissa [x]       = False
hissa (x:y:xs)  = if x == y && x == 's' then True else hissa (y:xs)


