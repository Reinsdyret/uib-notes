module Week42Exercise2 where


reverseWords :: String -> String
reverseWords = unwords . map reverse . words


sumIsEven :: Integer -> Integer -> Bool
sumIsEven = (even .) . (+) 
