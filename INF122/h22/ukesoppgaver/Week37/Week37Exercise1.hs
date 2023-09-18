module Week37Exercise1 where

checkZeroAndN :: [Integer] -> Integer -> Bool
checkZeroAndN abcd n
    | not $ all (>0) abcd = False
    | not $ all (<=n) abcd = False
    | otherwise = True

checkPowerThree :: Integer -> Integer -> Integer -> Integer -> Bool
checkPowerThree a b c d = a^3 + b^3 == c^3 + d^3

checkEquals :: Integer -> Integer -> Integer -> Integer -> Bool
checkEquals a b c d = a /= c && b /= d && a /= d && b /= c

checkAll :: Integer -> Integer -> Integer -> Integer -> Integer -> Bool
checkAll a b c d n = checkZeroAndN [a,b,c,d] n && checkPowerThree a b c d && checkEquals a b c d

equalCubeSum :: Integer -> [(Integer, Integer, Integer, Integer)]
equalCubeSum n = [(a,b,c,d) | a<- [1 .. n], b <- [1 .. n], c <- [1 .. n], d <- [1 .. n], checkAll a b c d n]

