module Week42Exercises where

-- Exercise 0
applyFunctions :: [a -> b] -> [b -> c] -> [a] -> [c]
applyFunctions (f:fs) (g:gs) (x:xs) = (g $ f x) : applyFunctions fs gs xs
applyFunctions _ _ _ = []

-- Exercise 1

fromLeftAndRight :: (Either a b -> c) -> (a -> c, b -> c)
fromLeftAndRight func = ((\x -> func (Left x)), (\y -> func (Right y)))

