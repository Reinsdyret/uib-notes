module Week43Exercise1 where

data RoseTree a = Branch a [RoseTree a]
  deriving (Eq, Show)

-- Example of an implementation like this used and found on:https://stackoverflow.com/questions/69695834/defining-fmap-for-a-rosetree
instance Functor RoseTree where
  fmap f (Branch a []) = Branch (f a) []
  fmap f (Branch a rest) = Branch (f a) (map (fmap f) rest)

sumNodes :: (Num a) => RoseTree [a] -> RoseTree a
sumNodes roseTree = fmap sum roseTree
