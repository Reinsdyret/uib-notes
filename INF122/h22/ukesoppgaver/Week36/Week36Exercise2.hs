module Week36Exercise2 where

import Data.Maybe (fromMaybe, isNothing)

halfPalindrome :: String -> Maybe String
halfPalindrome str
    | reverse str == str = Just $ take (div (length str) 2) str
    | otherwise = Nothing

getMiddleChar :: [Char] -> Char
getMiddleChar str = last $ take (length str - div (length str) 2) str

getValueFromMaybeStr :: Maybe String -> String
getValueFromMaybeStr = fromMaybe ""

decomposePalindrome :: String -> Maybe (String, Maybe Char)
decomposePalindrome str
    | isNothing $ halfPalindrome str = Nothing
    | even (length str) = Just (getValueFromMaybeStr $ halfPalindrome str, Nothing)
    | otherwise = Just (getValueFromMaybeStr $ halfPalindrome str, Just (getMiddleChar str))

createPalindrome :: String -> Maybe Char -> String
createPalindrome str ch 
    | isNothing ch = str ++ reverse str
    | otherwise = str ++ [fromMaybe 't' ch] ++ reverse str
