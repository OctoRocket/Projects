import Text.Printf

recursiveSqrt :: Integer -> Integer
recursiveSqrt num = worker 1 num
  where
    worker :: Integer -> Integer -> Integer
    worker n m = if ((n + 1) * (n + 1)) > m
      then n
      else worker (n + 1) m

factors :: Integer -> [(Integer, Integer)]
factors n = map (\x -> (x, div n x))
  $ filter (\x -> (==) 0 $ mod n x)
  [1..recursiveSqrt n]

main :: IO ()
main = do
  mapM_ (\x -> printf "%d x %d\n" (fst x) (snd x))
  $ factors 16
