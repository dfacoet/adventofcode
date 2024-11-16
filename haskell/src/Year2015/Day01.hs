module Year2015.Day01 (part1, part2) where

part1 :: String -> String
part1 input = show $ sum $ map evalChar input

part2 :: String -> String
part2 input =
  show $
    length $
      takeWhile
        (>= 0)
        (scanl (\f p -> f + evalChar p) 0 input) -- sequence of floors reached

evalChar :: Char -> Integer
evalChar c = case c of
  '(' -> 1
  ')' -> -1
  _ -> error "invalid character"
