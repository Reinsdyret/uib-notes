module NGram (NGram,Weight
             ,grams
             ,gramsWithNext
             ,combineGrams
             ,updateGram) where
import Control.Monad
import Data.List
import Control.Arrow

import qualified Data.Map as Map
import Data.Map (Map)

-- Rename types to clarify some type signatures later
type NGram = String
type Weight = Integer

-- Produce all n-grams contained in a given string
grams :: Integer -> String -> [NGram]
grams n [] = []
grams n string@(ch:chs)
  | (length string) < (fromIntegral n) = []
  | otherwise = (take (fromIntegral n) string) : grams n chs

-- Produce all n-grams contained in a given string, paired
-- with the subsequent character
gramsWithNext :: Integer -> String -> [(NGram,Char)]
gramsWithNext 0 _ = []
gramsWithNext _ [] = []
gramsWithNext n string = zip (grams n string) (map (last) (drop 1 (grams n string)))

-- Recombine a list of n-grams to a string
combineGrams :: [NGram] -> String
combineGrams [] = []
combineGrams (x:xs) = x ++ combineGrams' xs

combineGrams' :: [NGram] -> String
combineGrams' [] = []
combineGrams' (x:xs) = last x : combineGrams' xs

-- Update an n-gram by adding a character to the end
-- and removing the first character.
updateGram :: NGram -> Char -> NGram
updateGram g c = tail g ++ [c]

