module Model (TextModel
             ,createModel
             ,nextDistribution) where
import Control.Monad
import Data.List
import Control.Arrow

import qualified Data.Map as Map
import Data.Map (Map)

import NGram

-- The type for our Markov process text model.
type TextModel = Map NGram (Map Char Weight , Weight)

-- The empty model with no n-grams.
emptyModel :: TextModel
emptyModel = Map.empty

-- Update a model with a new n-gram followed by a character.
increaseWeight :: NGram -> Char -> TextModel -> TextModel
increaseWeight ngram next model
  | not (Map.member ngram model) = Map.insert ngram (Map.fromList [(next, 1)], 1) model 
  | otherwise = do
    let Just (charModel, weight) = Map.lookup ngram model
    let newNgramModel = Map.insertWith (+) next 1 charModel
    let newWeight = weight + 1
    Map.insert ngram (newNgramModel, newWeight) model

-- The distribution of next n-grams after a given one.
nextDistribution :: TextModel -> NGram -> Maybe ([(NGram, Weight)],Weight)
nextDistribution model current
  | Map.lookup current model == Nothing = Nothing
  | otherwise = do
    let Just (charModel, weight) = Map.lookup current model
    Just (helperNextDistribution charModel current, weight)



helperNextDistribution :: Map Char Weight -> NGram -> [(NGram, Integer)]
helperNextDistribution charModel ngram = map (helperCompleteTuple ngram) (Map.toList charModel)

helperCompleteTuple :: NGram -> (Char, Integer) -> (NGram, Integer)
helperCompleteTuple ngram2 (next, weight) = ((take 1 ngram2) ++ [next], weight)

-- Create an n-gram model from a string.
createModel :: Integer -> String -> TextModel
createModel n str = helperCreateModel (gramsWithNext n str) emptyModel 

helperCreateModel :: [(NGram, Char)] -> TextModel -> TextModel
helperCreateModel [] model = model
helperCreateModel ((ngram, char):rest) model = do
  let newModel = increaseWeight ngram char model
  helperCreateModel rest newModel

