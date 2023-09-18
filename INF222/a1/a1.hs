-- 1
f :: Num a => a -> a -> a
f x y = x + y

-- 2
sum_squares :: (Num a, Eq a) => a -> a
sum_squares 1 = 1
sum_squares x = x * x + sum_squares (x-1)

sum_squares' x = sum [a * a | a <- [0..x]]

sum_squares'' = \x -> sum [a * a | a <- [0..x]]


