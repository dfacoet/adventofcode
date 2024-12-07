module Year2024.Day04 (part1, part2) where

import           Data.List (transpose)

part1 :: String -> String
part1 input = show . length $ filter (\w -> w == "XMAS" || w == "SAMX") (allWords input)

part2 :: String -> String
part2 input = show . length $ filter isXMASBlock (allXBlocks input)

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

allXBlocks :: String -> [[String]]
allXBlocks input =
  let grid = lines input
   in [ map (take 3 . drop j) $ (take 3 . drop i) grid
        | i <- [0 .. length grid - 3],
          j <- [0 .. length (head grid) - 3]
      ]

isXMASBlock :: [String] -> Bool
isXMASBlock [[a, _, b], [_, 'A', _], [c, _, d]] = "MMSS" `elem` rotate [a, b, d, c]
isXMASBlock [_, _, _] = False
isXMASBlock _ = error "not a block"

rotate :: String -> [String]
rotate xs = [(take n . drop i) (cycle xs) | i <- [0 .. n - 1]]
  where
    n = length xs
