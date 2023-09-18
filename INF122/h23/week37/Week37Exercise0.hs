module Week37Exercise0 where

information :: [String] -> [String] -> [Integer] -> [String]
information [] _ _ = []
information (n:ns) (i:is) (y:ys) 
  | y < 2022 = information ns is ys
  | otherwise = (n ++ " is studying at " ++ i ++ " department and started in " ++ (show y)) : information ns is ys

