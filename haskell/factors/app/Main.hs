import Control.Monad (mapM_)

intSqrt :: Integer -> Integer
intSqrt n = intSqrtWorker n n

intSqrtWorker :: Integer -> Integer -> Integer
intSqrtWorker l n = if n * n <= l then n else intSqrtWorker l (n - 1)

factors :: Integer -> [(Integer, Integer)]
factors n = zip filtered $ map (div n) filtered
  where
    filtered = map fst
      $ filter snd
      $ zip rang
      $ (map ((==) 0)) 
      $ map (mod n) rang
    rang = [1..intSqrt n]

main :: IO ()
main = do
  mapM_ print $ factors 16
