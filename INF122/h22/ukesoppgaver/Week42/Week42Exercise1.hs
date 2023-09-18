module Week42Exercise1 where

import Data.Either

fromLeftAndRight :: (Either a b -> c) -> (a -> c, b -> c)
fromLeftAndRight original = (original.Left, original.Right)

either' :: (a -> c) -> (b -> c) -> Either a b -> c
either' = either

toFstAndSnd :: (a -> (b, c)) -> (a -> b, a -> c)
toFstAndSnd thing = (fst.thing, snd.thing)

pair :: (a -> b) -> (a -> c) -> a -> (b, c)
pair f g c = (f c, g c)
