module Main where

toLists :: (a, a) -> [a]
toLists t = [fst t, snd t]

merge :: (Ord a) => [a] -> [a] -> [a]
merge [] l = l
merge f [] = f
merge (f:fs) (l:ls) = if f < l
  then f : (merge fs $ l : ls)
  else l : (merge ls $ f : fs)

mergesort :: (Ord a) => [a] -> [a]
mergesort [] = []
mergesort l
  | length l == 1 = l
  | otherwise     = merge frst scnd
  where
    list = map mergesort $ toLists $ splitAt (div (length l) 2) l
    frst = head list
    scnd = last list

main :: IO ()
main = print $ mergesort [2, 3, 5, 1, 9, 6, 4, 8, 7]
