module Week41Exercise1 where

import Data.Map (Map)
import qualified Data.Map as Map
import Data.Set (Set)
import qualified Data.Set as Set

import Data.Maybe ( isNothing, fromJust )

type Graph n = Map n (Set n)


disjoint :: (Ord a) => Set a -> Set a -> Bool
disjoint a b = Set.null $ Set.intersection a b

hasCycle' :: (Ord n) => Graph n -> Set n -> n -> Bool
hasCycle' graph visited currentNode
    | isNothing (Map.lookup currentNode graph) = False
    | otherwise = do
        let newVisited = Set.insert currentNode visited
        let children = fromJust (Map.lookup currentNode graph)
        not (Set.null children) && not (disjoint newVisited children) || any (hasCycle' graph newVisited) (Set.toList children)


hasCycle :: (Ord n) => Graph n -> n -> Bool
hasCycle graph = hasCycle' graph Set.empty