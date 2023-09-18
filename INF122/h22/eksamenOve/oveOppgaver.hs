{-# LANGUAGE ParallelListComp #-}
-- Listeoperasjoner d
differences list = [a-b | a<-list, b<-list, a > b ]

everyOther :: [a] -> [a]
everyOther [] = []
everyOther [a] = []
everyOther (x:_:xs) = x : everyOther xs
