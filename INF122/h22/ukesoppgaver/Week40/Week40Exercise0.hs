module Week40Exercise0 where
import Data.Maybe (isNothing, fromMaybe)


data Palindrome = Palindrome{
    body :: String,
    middleChar :: Maybe String
} deriving Show


-- HalfPalindrome funksjon fra Uke 36 Oppgave 2
halfPalindrome :: String -> Maybe String
halfPalindrome str
    | reverse str == str = Just $ take (div (length str) 2) str
    | otherwise = Nothing

-- GetMiddleChar funksjon fra Uke 36 Oppgave 2
getMiddleChar :: [Char] -> Maybe String
getMiddleChar str
    | even (length str) = Nothing
    | otherwise = Just (take 1 $ drop (div (length str) 2) str)

-- CreatePalindrome funksjon fra Uke 36 Oppgave 2
createPalindrome :: String -> Maybe String -> String
createPalindrome str ch = str ++ fromMaybe "" ch ++ reverse str

palindrome :: String -> Maybe Palindrome
palindrome str
    | isNothing (halfPalindrome str) = Nothing
    | otherwise =
        if null str then
            Just (Palindrome (fromMaybe "" (halfPalindrome str)) (Just ""))
        else
            Just (Palindrome (fromMaybe "" (halfPalindrome str)) (getMiddleChar str))

toString :: Palindrome -> String
toString pal = createPalindrome (body pal) (middleChar pal)
