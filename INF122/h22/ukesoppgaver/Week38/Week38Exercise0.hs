module Week38Exercise0 where

duplicateAll :: [a] -> [(a,a)]
duplicateAll [] = []
duplicateAll (x:xs) = [(x,x)] ++ duplicateAll xs