module Main where

threeXPlusOne :: Integer -> [Integer]
threeXPlusOne 1 = [1]
threeXPlusOne n = n : threeXPlusOne next_step
  where
    next_step = if mod n 2 == 0 then n `div` 2 else n * 3 + 1

main :: IO ()

main = mapM_ (\l -> print $
              show (fst l)          ++
              ": "                  ++
              show (length $ snd l) ++
              ", "                  ++
              show (snd l)            )
  $ zip [1 :: Integer ..] $ map threeXPlusOne [1..10]