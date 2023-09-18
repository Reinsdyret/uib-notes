module Week40Exercise2 where

data BinSearchTree a
  = Empty
  | Branch (BinSearchTree a) a (BinSearchTree a)
  deriving (Eq, Show)

toList :: BinSearchTree a -> [a]
toList Empty = []
toList (Branch left root right) = toList left ++ [root] ++ toList right
