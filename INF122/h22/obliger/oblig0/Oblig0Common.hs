module Oblig0Common where

-- Compute the average of a list of values
average :: (Fractional a) => [a] -> a
average list = sum list / fromIntegral (length list)

-- A simple lowpass filter with adjustable cut-off
lpf :: (Fractional a) => Integer -> [a] -> a
lpf n list = average (take (fromIntegral n) list)

-- A simple high pass filter with adjustable cut-off
hpf :: (Floating a) => Integer -> [a] -> a
hpf _ [] = 0
hpf n (x:xs) = x - lpf n (x:xs)

-- Extend a finite signal with an infinite constant past
extend ::  Num a => [a] -> [a]
extend [] = repeat 0
extend [x] = repeat x
extend (x:xs) = x : extend xs


-- Apply a filter to a list of values
applyFilter :: (Num a, Floating a) => ([a] -> a) -> [a] -> [a]
applyFilter fil = map fil . iterate tail . extend

-- Returns if the two given values are a zeroCrossing
zeroCrossing :: (Num a, Ord a) => a -> a -> Integer
zeroCrossing a b
    | a > 0 && b <= 0 || b > 0 && a <= 0 = 1
    | otherwise = 0


-- Count the number of zero-crossings in a signal represented by a list
zeroCrossings :: (Num a, Ord a) => [a] -> Integer
zeroCrossings [] = 0
zeroCrossings [x] = 0
zeroCrossings list = zeroCrossing (head list) (list !! 1) + zeroCrossings (drop 1 list)

lowPassCutoff :: Integer
lowPassCutoff = 88
highPassCutoff :: Integer
highPassCutoff = 85
