module Week37Exercise0 where

createStr :: String -> Integer -> String
createStr name age = name ++ " is " ++ show age ++ " years old"


namesAndAges :: [String] -> [Integer] -> [String]
namesAndAges names ages = map (uncurry createStr) (filter ((<=50).snd) (zip names ages))