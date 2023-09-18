main = getLine >>= compute . read

pos :: [Int]
pos = [1..]

oddInts :: [Int]
oddInts = [x | x<-[1..], odd x]

evenInts :: [Int]
evenInts = [x | x<-[1..], even x]

sst :: Int -> [Int] -> String
sst t list = show $ sum $ take t list

compute :: Int -> IO ()
compute 0 = return ()
compute n = do
  line <- getLine
  let [p, t] = map read (words line)
  putStrLn ((show p) ++ " " ++ (sst t pos) ++ " " ++ (sst t oddInts) ++ " " ++ (sst t evenInts))
  compute (n-1)

