module Week36Exercise2 where

semiRepetitive :: String -> Maybe String
semiRepetitive str 
  | even $ length str = let halfLen = div (length str) 2
                        in 
                        if take halfLen str == drop halfLen str 
                          then Just (take halfLen str)
                          else Nothing
  | otherwise = let halfLen = div (length str) 2
                in 
                if take halfLen str == drop (halfLen + 1) str 
                  then Just (take halfLen str)
                  else Nothing

decomposeSemiRepetitive :: String -> Maybe (String, Maybe Char)
decomposeSemiRepetitive str 
  | even $ length str = if semiRepetitive str == Nothing
                              then Nothing
                              else
                                let Just a = semiRepetitive str 
                                in Just (a, Nothing)
  | otherwise = if semiRepetitive str == Nothing then Nothing
                else 
                  let Just a = semiRepetitive str 
                  in Just (a, Just $ head $ drop (length a) str)

createSemiRepetitive :: String -> Maybe Char -> String
createSemiRepetitive str Nothing = str ++ str
createSemiRepetitive str (Just a) = str ++ [a] ++ str


getMiddleChar :: String -> Char
getMiddleChar str = let halfLen = div (length str) 2
                    in head $ drop halfLen str 

isSemiRepetitive :: String -> Bool
isSemiRepetitive str 
  | even $ length str =
    let halfLen = div (length str) 2
    in take halfLen str == drop halfLen str 

  | otherwise = let halfLen = div (length str) 2 
    in take halfLen str == drop (halfLen + 1) str

decomposeSemiRepetitive' :: String -> Maybe (String, Maybe Char)
decomposeSemiRepetitive' str 
  | not $ isSemiRepetitive str = Nothing
  | even $ length str = let Just a = semiRepetitive str 
    in Just (a, Nothing)
  | otherwise = 
    let Just a = semiRepetitive str 
    in Just (a, Just $ getMiddleChar str)


