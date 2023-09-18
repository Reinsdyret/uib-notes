module Week38Exercise2 where
import Data.Maybe (isNothing, fromJust)

removeNothing :: [Maybe a] -> [a]
removeNothing [] = []
removeNothing (x:xs)
    | isNothing x = [] ++ removeNothing xs
    | otherwise = [fromJust x] ++ removeNothing xs