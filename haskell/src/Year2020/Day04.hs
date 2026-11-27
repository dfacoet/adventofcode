module Year2020.Day04 (part1, part2) where

import Data.List (stripPrefix)
import Data.List.Split (endByOneOf, splitOn)
import Data.Map (Map, fromList, keys, lookup)
import Data.Set (fromList, isSubsetOf, member)
import Text.Read (readMaybe)

part1 :: String -> String
part1 = show . length . filter isValidOne . parseInput

part2 :: String -> String
part2 = show . length . filter isValidTwo . parseInput

type Passport = Map String String

parseInput :: String -> [Passport]
parseInput = map parsePassport . splitOn "\n\n"

parsePassport :: String -> Passport
parsePassport s = Data.Map.fromList (map parseField (endByOneOf [' ', '\n'] s))

parseField :: String -> (String, String)
parseField x = case Data.List.Split.splitOn ":" x of
  [k, v] -> (k, v)
  _ -> error "Invalid input"

isValidOne :: Passport -> Bool
isValidOne p = expected `isSubsetOf` Data.Set.fromList (keys p)
  where
    expected =
      Data.Set.fromList
        [ "byr",
          "iyr",
          "eyr",
          "hgt",
          "hcl",
          "ecl",
          "pid"
        ]

isValidTwo :: Passport -> Bool
isValidTwo p =
  validYear "byr" 1920 2002 p
    && validYear "iyr" 2010 2020 p
    && validYear "eyr" 2020 2030 p
    && validHeight p
    && validHair p
    && validEye p
    && validPid p

validNumber :: String -> Int -> Int -> Bool
validNumber s a b = case readMaybe s of
  Just x | x >= a && x <= b -> True
  _ -> False

validYear :: String -> Int -> Int -> Passport -> Bool
validYear k a b p = case Data.Map.lookup k p of
  Just y -> validNumber y a b
  _ -> False

stripSuffix :: String -> String -> Maybe String
stripSuffix suf s = reverse <$> stripPrefix (reverse suf) (reverse s)

validHeight :: Passport -> Bool
validHeight p = case Data.Map.lookup "hgt" p of
  Just s | Just c <- stripSuffix "cm" s -> validNumber c 150 193
  Just s | Just i <- stripSuffix "in" s -> validNumber i 59 76
  _ -> False

validHair :: Passport -> Bool
validHair p = case Data.Map.lookup "hcl" p of
  Just ('#' : xs) -> (length xs == 6) && Data.Set.fromList xs `isSubsetOf` expected
    where
      expected = Data.Set.fromList "0123456789abcdef"
  _ -> False

validEye :: Passport -> Bool
validEye p = case Data.Map.lookup "ecl" p of
  Just s -> s `Data.Set.member` Data.Set.fromList ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
  _ -> False

validPid :: Passport -> Bool
validPid p = case Data.Map.lookup "pid" p of
  Just s | length s == 9 -> case (readMaybe s :: Maybe Int) of
    Just _ -> True
    Nothing -> False
  _ -> False

-- validField :: String -> (String -> Bool) -> Passport -> Bool
-- validField k condition p = maybe False condition (Data.Map.lookup k p)
