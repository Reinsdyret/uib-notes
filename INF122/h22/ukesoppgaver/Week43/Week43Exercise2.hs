{-# LANGUAGE FlexibleInstances #-}
module Week43Exercise2 where

class IntegerGraph g where
  emptyGraph :: g
  insertNode :: Integer -> g -> g
  insertEdge :: Integer -> Integer -> g -> g
  nodeInGraph :: Integer -> g -> Bool
  edgeInGraph :: Integer -> Integer -> g -> Bool

data MyGraph = MyGraph [Integer] [(Integer, Integer)]
  deriving (Show, Eq)

instance IntegerGraph MyGraph where
  emptyGraph = MyGraph [] []
  insertNode u (MyGraph nodes edges) = MyGraph (u : nodes) edges
  insertEdge u v (MyGraph nodes edges) = MyGraph nodes ((u,v) : edges)
  nodeInGraph u (MyGraph nodes edges) = elem u nodes
  edgeInGraph u v (MyGraph nodes edges) = elem (u,v) edges

helperAddEdgeFromTuple :: (IntegerGraph g) => (Integer, Integer) -> g -> g
helperAddEdgeFromTuple (u,v) graph = insertEdge u v graph

graph :: (IntegerGraph g) => g
graph = let graphWithNodes = foldr insertNode emptyGraph [1,5,3,6,8]
 in foldr helperAddEdgeFromTuple graphWithNodes [(1,8),(1,6),(5,1),(5,8),(8,5)]
