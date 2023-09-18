-- a
-- map (+3) [1,2,3] = [4,5,6]
--
-- b
-- sum [x+3 | x <- [1,2,3]] = sum [4,5,6] = 15
--
-- c
-- (\x y -> x-y) 7 3 = 7-3 = 4
--
-- d
-- :t (\x -> 3 : tail x) = [Int] -> [Int] eller mer generelt = Num a => [a] -> [a]
--
-- e
-- :t (3, Just "Haskell") = (Integer, Maybe String) eller mer generelt = Num a => (a, Maybe String)
--
-- f
-- :t Just (Left Nothing) = Maybe (Either (Maybe a) b)
--
-- g
-- :k Either String = * -> *
--
-- h
-- :k Integer -> Integer = * -> * FEILLLL DET ER *
--
-- f x y = if x then y else y*3
-- :t f = Bool -> Integer -> Integer
-- :t f = (Num a) => Bool -> a -> a
-- :t f = (Eq a) => a -> Integer -> Integer
--
import qualified Data.Map as Map
import Data.Maybe (isNothing)
-- ENKEL IO
readName :: IO ()
readName = do
  name <- getLine
  let [firstName, lastName] = words name
  putStrLn $ lastName ++ ", " ++ firstName

isConsonant :: Char -> Bool
isConsonant char = elem char "bcdfghjklmnpqrstvwxz"

translate :: String -> String
translate word = do
  x <- word
  if isConsonant x then [x] ++ "o" ++ [x] else [x]
--translate = concat . map (\x -> if isConsonant x then [x] ++ "o" ++ [x] else [x])
--translate word = concat [ if isConsonant x then [x] ++ "o" ++ [x] else [x] | x <- word]


differences :: [Integer] -> [Integer]
differences list = [a - b | a<-list, b<-list, a > b]


everyOther :: [a] -> [a]
everyOther [] = []
everyOther [x] = x : []
everyOther (x:_:xs) = x : everyOther xs


type Graph label node = Map.Map node (Map.Map label node)
data N = A | B | C


graph0 :: Graph Char Int
graph0 = Map.fromList [(1,Map.fromList [('r',2)])
                      ,(2,Map.fromList [('o',2),('t',3)])
                      ,(3,Map.fromList [('e',1),('t',3)])]

-- insertLabeledEdge :: Graph label node -> node -> node -> label -> Graph label node
-- insertLabeledEdge graph nodeA nodeB edgeLabel = Map.insert nodeA  (Map.fromList [(edgeLabel, nodeB)]) graph

goNext :: (Ord label, Ord node) => Graph label node -> node -> label -> Maybe node
goNext graph start label = Map.lookup start graph >>= Map.lookup label >>= return
-- goNext graph start label = do
--  connectingNodes <- Map.lookup start graph
--  next <- Map.lookup label connectingNodes
--  return next

followPath :: (Ord label, Ord node) => Graph label node -> node -> [label] -> Maybe node
followPath graph current []             = return current
followPath graph current (label:labels) = do
  connecting <- Map.lookup current graph
  next <- Map.lookup label connecting
  followPath graph next labels

