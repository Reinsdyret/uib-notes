module Main where

-- Statements
data Stmt
  = Skip
  | Assign String Expr
  | Seq Stmt Stmt
  | If Expr Stmt Stmt
  | While Expr Stmt
--  deriving Show

instance Show Stmt where
  show  (Skip) = "; "
  show (Assign s e) = id s ++ " = " ++ show e ++ "; "
  show (Seq s1 s2) = show s1 ++ " " ++ show s2
  show (If e1 s1 s2) = "if (" ++ show e1 ++ ") { " ++ show s1 ++ " } else { " ++ show s2 ++ " }; "
  show (While e s) = "while (" ++ show e ++ ") { " ++ show s ++ " }; "

-- Expressions
data Expr
  = IntConst Int
  | BoolConst Bool
  | Var String
  | Unary UOp Expr
  | Binary BOp Expr Expr
--  deriving Show

instance Show Expr where
  show (IntConst i) = show i
  show (BoolConst b) = show b
  show (Var s) = id s
  show (Unary u e) = show u ++ "(" ++ show e ++ ")"
  show (Binary b e1 e2) = "(" ++ show e1 ++ ")" ++ show b ++ "(" ++ show e2 ++ ")"

-- Unary and binary operators
data UOp
  = Negate
  | Not
--  deriving Show
instance Show UOp where
  show (Negate) = "-"
  show (Not) = "!"
  
data BOp
  = Add
  | Sub
  | Mul
  | Lt
  | Leq
  | Eq
  | Geq
  | Gt
  | And
  | Or
--  deriving Show
instance Show BOp where
  show (Add) = "+"
  show (Sub) = "-"
  show (Mul) = "*"
  show (Lt) = "<"
  show (Leq) = "<="
  show (Eq) = "=="
  show (Geq) = ">="
  show (Gt) = ">"
  show (And) = "&&"
  show (Or) = "||"


main = do 
  let myProgram = Seq (Assign "x" (IntConst 10)) (If (Binary Gt (Var "x") (IntConst 0)) (Assign ("x") (IntConst 5)) (Skip)) in
    print myProgram
