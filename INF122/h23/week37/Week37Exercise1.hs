module Week37Exercise1 where

-- This function computes the (m-1)-th root of a number.
root :: Integer -> Integer -> Double
root m x = fromIntegral x ** (1 / fromIntegral (m - 1))

-- This function determines if a number is close enough to an integer.
isNearInteger :: Double -> Bool
isNearInteger x = abs (x - (fromIntegral . round) x) < 1e-10

-- The main function to solve the problem.
semiFermat :: Integer -> Integer -> [(Integer, Integer, Integer)]
semiFermat n m = [(a, b, round c) | 
                    a <- [1..n], 
                    b <- [a..n], 
                    let c = root m (a^m + b^m), 
                    isNearInteger c, 
                    c >= fromIntegral a && c >= fromIntegral b && c <= fromIntegral n]

