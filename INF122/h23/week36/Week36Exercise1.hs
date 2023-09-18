module Week36Exercise1 where

f :: [Integer] -> [t] -> [(Integer, t)]
f l1 l2 = zip l1 $ reverse l2

