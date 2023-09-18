module Week41Exercise2 where

import Data.Map (Map)
import qualified Data.Map as Map
import Data.Maybe (isNothing)

data Expr a
  = Var a
  | Lit Integer
  | Mul (Expr a) (Expr a)
  | Add (Expr a) (Expr a)
  deriving (Eq, Show)

eval :: (Ord variable, Num value) => Expr variable -> Map variable value -> Maybe value
eval (Lit num) _ = Just (fromIntegral num)
eval (Var a) def = Map.lookup a def
eval (Mul left right) def
  | isNothing (eval left def) || isNothing (eval right def) = Nothing
  | otherwise = do
    let Just a = eval left def
    let Just b = eval right def
    Just (a * b)

eval (Add left right) def
  | isNothing (eval left def) || isNothing (eval right def) = Nothing
  | otherwise = do
    let Just a = eval left def
    let Just b = eval right def
    Just (a + b)
