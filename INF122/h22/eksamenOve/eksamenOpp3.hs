import Data.Char (toLower, toUpper)

main :: IO ()
main = do
    name <- getLine
    let lowerReversedName = map toLower (reverse' name)
    putStrLn $ "Hei, " ++ [toUpper $ head lowerReversedName] ++ (tail lowerReversedName) ++ "!"
    
    
-- Reverses a string
reverse' :: String -> String
reverse' [] = []
reverse' str = (last str) : reverse' (init str)
