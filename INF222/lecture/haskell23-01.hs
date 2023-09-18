module Naturals where

data Natural = Zero | Succ Natural
  deriving (Show)

plus :: Natural -> Natural -> Natural
plus Zero n = n
plus (Succ m) n = Succ (plus m n)

mult :: Natural -> Natural -> Natural
mult Zero n = Zero
mult (Succ m) n = plus n (mult m n)

sub :: Natural -> Natural -> Maybe Natural
sub Zero Zero = Just Zero
sub Zero n = Nothing
sub n Zero = Just n
sub (Succ m) (Succ n) = sub m n

natToInt :: Natural -> Integer
natToInt Zero = 0
natToInt (Succ m) = 1 + natToInt m
