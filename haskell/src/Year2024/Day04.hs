module Year2024.Day04 (part1, part2) where

import Data.List (transpose)

part1 :: String -> String
part1 input = show . length $ filter (\w -> w == "XMAS" || w == "SAMX") (allWords input)

part2 :: String -> String
part2 = error "Part2 not implemented"

allWords :: String -> [String]
allWords grid =
  concat
    ( sequenceA
        [horizontalWords, verticalWords, diagWords, antiDiagWords]
        (lines grid)
    )

horizontalWords :: [String] -> [String]
horizontalWords = concatMap (\row -> [(take 4 . drop i) row | i <- [0 .. length row - 4]])

verticalWords :: [String] -> [String]
verticalWords = horizontalWords . transpose

diagWords :: [String] -> [String]
diagWords grid =
  [ diagIdx <*> [map (drop j) $ (take 4 . drop i) grid]
    | i <- [0 .. length grid - 4],
      j <- [0 .. length (head grid) - 4]
  ]
  where
    diagIdx :: [[[a]] -> a]
    diagIdx = map (\i xss -> (xss !! i) !! i) [0 .. 3]

antiDiagWords :: [String] -> [String]
antiDiagWords = diagWords . map reverse
