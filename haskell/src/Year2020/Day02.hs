{-# OPTIONS_GHC -Wno-name-shadowing #-}

module Year2020.Day02 (part1, part2) where

part1 :: String -> String
part1 input = show $ length $ filter isValidOne (parseInput input)

part2 :: String -> String
part2 input = show $ length $ filter isValidTwo (parseInput input)

data Policy = Policy
  { a :: Int,
    b :: Int,
    c :: Char,
    password :: String
  }
  deriving (Show)

parsePolicy :: String -> Maybe Policy
parsePolicy s =
  case break (== '-') s of
    (minString, '-' : r1) ->
      case break (== ' ') r1 of
        (maxString, ' ' : r2) ->
          case break (== ':') r2 of
            ([c], ':' : ' ' : p) ->
              Just
                Policy
                  { a = read minString,
                    b = read maxString,
                    c = c,
                    password = p
                  }
            _ -> Nothing
        _ -> Nothing
    _ -> Nothing

parseInput :: String -> [Policy]
parseInput i = case mapM parsePolicy (lines i) of
  Just xs -> xs
  Nothing -> error "Invalid input"

isValidOne :: Policy -> Bool
isValidOne Policy {a = a, b = b, c = c, password = password} = a <= l && l <= b
  where
    l = length $ filter (== c) password

isValidTwo :: Policy -> Bool
isValidTwo Policy {a = a, b = b, c = c, password = password} =
  ((password !! (a - 1)) == c)
    /= ((password !! (b - 1)) == c)
