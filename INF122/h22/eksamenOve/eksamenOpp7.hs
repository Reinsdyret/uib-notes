import Data.Map (Map)
import Data.List
import qualified Data.Map as Map

data Record key value
  = Record (Map key (Either (Record key value) value))

x :: Record String Integer
x = Record
  $ Map.fromList [("A",Right 3)

                             ,("B",Left (Record (Map.singleton "C" (Right 3))))]

y :: Record String Integer
y = Record
  $ Map.fromList [("A", Right 4)
                 ,("B", Left (Record (Map.singleton "D" (Right 3))))]

instance (Show key, Show value) => Show (Record key value) where
  show (Record m)
      = "{" ++ concat (intersperse ";" [ show k ++ ":"  ++ either show show v
                 | (k,v) <- Map.toList m]) ++ "}"



-- a)
shallowUnion :: (Ord key) => Record key value -> Record key value -> Record key value
shallowUnion (Record rec1) (Record rec2) = Record $ Map.union rec1 rec2


-- b)
deepUnion :: (Ord key) => Record key value -> Record key value -> Record key value
deepUnion (Record rec1) (Record rec2) = Record $ Map.unionWith (unpackAndUnion) rec1 rec2

-- Hjelpefunksjon for å patternmatche hvis begge verdiene er en record (Left)
unpackAndUnion ::(Ord key) => Either (Record key value) value -> Either (Record key value) value -> Either (Record key value) value
unpackAndUnion (Left rec1) (Left rec2) = Left $ deepUnion rec1 rec2
unpackAndUnion l _ = l

