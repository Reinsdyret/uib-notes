
module TestCaeser where

import qualified Data.Set as Set
import qualified Data.Map as Map
import Data.List (elemIndex, sort, sortBy, minimumBy, maximumBy)

type Key = [(Char,Char)]
type FrequencyTable = [(Char,Double)]
type Alphabet = String
type Dictionary = Set.Set String

caesar :: Alphabet -> Int -> Key
caesar alphabet n = zip alphabet (drop (fromIntegral n) (cycle alphabet))
