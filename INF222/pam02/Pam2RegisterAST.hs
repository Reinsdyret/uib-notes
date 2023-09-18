-- | AST for register based integer calculator.
--
-- Author Magne Haveraaen
-- Since 2020-03-14

module Pam2RegisterAST where
import Pam2RegisterStore 
import Test.QuickCheck
import GHC.Data.BooleanFormula (eval)

-----------------------

-- | Expressions for a calculator with 10 registers.
-- The calculator supports literals and operations
-- Addition, multiplication, and subtraction/negation.
data CalcExprAST
  = Lit Integer
  | Add CalcExprAST CalcExprAST
  | Mult CalcExprAST CalcExprAST
  | Sub CalcExprAST CalcExprAST
  | Neg CalcExprAST
  | Reg Register
  deriving (Eq, Read, Show)

-- | Statement for setting a register
data CalcStmtAST
  = SetReg Register CalcExprAST
  deriving (Eq, Read, Show)

-- | Enumeration of the 10 registers.
data Register
  = Reg0
  | Reg1
  | Reg2
  | Reg3
  | Reg4
  | Reg5
  | Reg6
  | Reg7
  | Reg8
  | Reg9
  deriving (Eq, Read, Show)



-----------------------

-- | A few ASTs for register based CalcExprAST.
calculatorRegisterAST1
  = Lit 4
calculatorRegisterAST2
  = Neg (Mult (Add (Lit 3) (Sub (Lit 7) (Lit 13))) (Lit 19))
calculatorRegisterAST3
  = Add (Reg Reg1) (Reg Reg4)
calculatorRegisterAST4
  = Reg Reg2
  
-- | A few ASTs for setting registers CalcStmtAST.
calculatorSetRegisterAST1
  = SetReg Reg4 calculatorRegisterAST1
calculatorSetRegisterAST2
  = SetReg Reg1 calculatorRegisterAST2
calculatorSetRegisterAST3
  = SetReg Reg2 calculatorRegisterAST3
calculatorSetRegisterAST4
  = SetReg Reg1 calculatorRegisterAST4

-----------------------

-- Evaluate a calculator expression given a Store
evaluate :: CalcExprAST -> Store -> Integer
evaluate (Reg a) store = getStore store $ getRegisterIndex a
evaluate (Lit a) store = a
evaluate (Add a b) store = (evaluate a store) + (evaluate b store)
evaluate (Mult a b) store = (evaluate a store) * (evaluate b store)
evaluate (Sub a b) store = (evaluate a store) - (evaluate b store)
evaluate (Neg a) store = (-1) * (evaluate a store)

-- Set the value of a calculator expression to a register in the store
execute :: CalcStmtAST -> Store -> Store
execute (SetReg reg stmt) store = setStore (getRegisterIndex reg) (evaluate stmt store) store

-- Map a register to an index in the store
getRegisterIndex :: Register -> Integer
getRegisterIndex a = case a of
  Reg0 -> 0
  Reg1 -> 1
  Reg2 -> 2
  Reg3 -> 3
  Reg4 -> 4
  Reg5 -> 5
  Reg6 -> 6
  Reg7 -> 7
  Reg8 -> 8
  Reg9 -> 9


-- Tests
litTest :: Integer -> Bool
litTest x = evaluate (Lit x) registerStore == x

addTest :: Integer -> Integer -> Bool
addTest x y = evaluate (Add (Lit x) (Lit y)) registerStore == x + y


main :: IO ()
main = do
  putStrLn "Testing litTest ..."
  quickCheck litTest

  putStrLn "Testing addTest ..."
  quickCheck addTest

