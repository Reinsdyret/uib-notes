main = 
  getLine >>= \line ->
  if leftOfEye line == (rightOfEye $ strRightOfEye line)
  then putStrLn "correct"
  else putStrLn "fix"
  
leftOfEye :: String -> Int
leftOfEye ('(':xs)     = 0
leftOfEye (_:xs)       = 1 + leftOfEye xs

rightOfEye :: String -> Int
rightOfEye []          = 0
rightOfEye (_:xs)      = 1 + rightOfEye xs
 
strRightOfEye :: String -> String
strRightOfEye (')':xs) = xs
strRightOfEye (_:xs)   = strRightOfEye xs
