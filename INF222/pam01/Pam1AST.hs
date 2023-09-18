-- | AST for simple integer calculator.
--
-- Author Magne Haveraaen
-- Since 2020-03-14

module Pam1AST where


-----------------------

-- | Expressions for a simple calculator.
-- The calculator supports literals and operations
-- Addition, multiplication, and subtraction/negation.
data CalcExprAST
  = Lit Integer
  | Add CalcExprAST CalcExprAST
  | Mult CalcExprAST CalcExprAST
  | Sub CalcExprAST CalcExprAST
  | Neg CalcExprAST
  deriving (Eq, Read, Show)


-----------------------

-- | A couple ASTs for CalcExprAST.
calculatorAST1 = Lit 4
calculatorAST2
  = Neg (Mult (Add (Lit 3) (Sub (Lit 7) (Lit 13))) (Lit 19))

-----------------------

-- Interpreter for the CalcExprAST type to an integer

eval :: CalcExprAST -> Integer
eval (Lit a) = a
eval (Add a b) = (eval a) + (eval b)
eval (Mult a b) = (eval a) * (eval b)
eval (Sub a b) = (eval a) - (eval b)
eval (Neg a) = (-1) * (eval a)
            
          
        
      
    
  

