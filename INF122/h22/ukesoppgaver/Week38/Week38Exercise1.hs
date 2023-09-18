module Week38Exercise1 where

mySplitAt :: Integer -> [a] -> ([a],[a])
mySplitAt n a
    | n < 1 = ([], a)
    | otherwise = mySplitAt' (n-1) (take 1 a, drop 1 a)

mySplitAt' 0 tuple = tuple
mySplitAt' n (a, b) = mySplitAt' (n-1) (a ++ take 1 b, drop 1 b)