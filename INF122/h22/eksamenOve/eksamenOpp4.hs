-- a)
sumOfSquares :: [Integer] -> Integer
sumOfSquares nums = sum [x * x | x<-nums]

-- b)
sumOfSquares' :: [Integer] -> Integer
sumOfSquares' = sum . map (\x -> x * x)

-- c)
duplicate :: [a] -> [a]
duplicate [] = []
duplicate (x:xs) = x : x : duplicate xs

-- d)
--"bdf"
-- f "abcdefg"
-- map snd . filter (odd . fst) . zip [0..] "abcdefg"
-- zip [0..] "abcdefg" = [(0,'a'),(1,'b'),(2,'c'),(3,'d'),(4,'e'),(5,'f'),(6,'g')]
-- filter (odd . fst) [(0,'a'),(1,'b'),(2,'c'),(3,'d'),(4,'e'),(5,'f'),(6,'g')] = [(1,'b'),(3,'d'),(5,'f')]
-- map snd [(1,'b'),(3,'d'),(5,'f')] = "bdf"

-- "bdf" er svaret
