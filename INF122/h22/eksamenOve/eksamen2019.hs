-- Oppgave 2

-- 2.1

row :: [[Int]] -> Int -> [Int]
row m r = m !! (r-1)


col :: [[Int]] -> Int -> [Int]
col m k = map (!! (k-1)) m


cols :: [[Int]] -> [[Int]]
cols [] = []
cols m = [col m k | k<-[1..(length m)]]


mult :: [[Int]] -> [[Int]] -> [[Int]]
mult m n = chunksOf (map mult' [(a,b) | a<-m, b<-cols n]) (length m)

mult' :: ([Int], [Int]) -> Int
mult' ([],[]) = 0
mult' ((x:xs), (y:ys)) = x*y + mult' (xs,ys)

chunksOf :: [a] -> Int -> [[a]]
chunksOf [] _ = []
chunksOf l n = take n l : chunksOf (drop n l) n


