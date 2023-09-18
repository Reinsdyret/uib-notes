module Week43Exercise0 where

data BinSearchTree a
  = Empty
  | Branch (BinSearchTree a) a (BinSearchTree a)
  deriving (Eq, Show)

instance Foldable BinSearchTree where
  foldr _ x Empty = x
  foldr f x (Branch left a right) = foldr f (f a (foldr f x right)) left

toList :: BinSearchTree a -> [a]
toList bin = foldr (:) [] bin
