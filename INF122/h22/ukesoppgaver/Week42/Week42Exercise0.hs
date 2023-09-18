module Week42Exercise0 where

applyFunctions :: [a -> b] -> [a] -> [b]
applyFunctions [] _ = []
applyFunctions _ [] = []
applyFunctions (x:xs) (y:ys) = x y : applyFunctions xs ys