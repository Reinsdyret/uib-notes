module Week41Exercise0 where

import Data.Map (Map)
import qualified Data.Map as Map
import Data.Set (Set)
import qualified Data.Set as Set

type Graph n = Map n (Set n)

insertEdge :: (Ord n) => n -> n -> Graph n -> Graph n
insertEdge u v graph
    | Map.member u graph && Map.member v graph = Map.insertWith Set.union u (Set.singleton v) graph
    | Map.member u graph = do
        let newGraph = Map.insert v Set.empty graph
        Map.insertWith Set.union u (Set.singleton v) newGraph
    | Map.member v graph = do
        let newGraph = Map.insert u Set.empty graph
        Map.insertWith Set.union u (Set.singleton v) newGraph
    | otherwise = do
        let newGraph = Map.insert u Set.empty graph
        let newGraph2 = Map.insert v Set.empty newGraph
        Map.insertWith Set.union u (Set.singleton v) newGraph2
