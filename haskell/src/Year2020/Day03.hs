module Year2020.Day03 (part1, part2) where

part1 :: String -> String
part1 input = show $ traverseSlope (lines input) 3 1

part2 :: String -> String
part2 input =
  show $
    product
      ( map
          (uncurry (traverseSlope (lines input)))
          [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
      )

traverseSlope :: [String] -> Int -> Int -> Int
traverseSlope slope right 1 =
  length $
    filter
      (== '#')
      [row !! (right * k `mod` l) | (k, row) <- zip [0 ..] slope]
  where
    l = length $ head slope
traverseSlope slope 1 down =
  length $
    filter
      (== '#')
      [row !! (k `mod` l) | (k, row) <- zip [0 ..] (every down slope)]
  where
    l = length $ head slope
traverseSlope _ _ _ = error "not implemented"

every :: Int -> [a] -> [a]
every n (x : xs) = x : every n (drop (n - 1) xs)
every _ [] = []
