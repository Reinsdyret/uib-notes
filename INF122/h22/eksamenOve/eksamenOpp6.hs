-- c)
hasLength :: Int -> [a] -> Bool
hasLength 0 [] = True
hasLength 0 (x:xs) = False
hasLength n (x:xs) = hasLength (n-1) xs
