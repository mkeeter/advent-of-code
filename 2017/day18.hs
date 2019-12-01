import Text.Read (readMaybe)
import Data.Maybe
import Data.Char
import Data.List.Split
import qualified Data.Vector as V
import qualified Data.Map as M

type Reg = Char
data Value = Value Int | Register Reg deriving Show
data Instruction = Send Value
                 | Set Reg Value
                 | Add Reg Value
                 | Mul Reg Value
                 | Mod Reg Value
                 | Receive Reg
                 | Jump Value Value deriving Show

readRegister = head . dropWhile isSpace
readValue s =
  let r = Register $ readRegister s
      v = Value <$> readMaybe s
    in fromMaybe r v

readLine :: String -> Instruction
readLine s = case i of
  "snd" -> Send (readValue $ args !! 0)
  "set" -> Set (readRegister $ args !! 0)
               (readValue $ args !! 1)
  "add" -> Add (readRegister $ args !! 0)
               (readValue $ args !! 1)
  "mul" -> Mul (readRegister $ args !! 0)
               (readValue $ args !! 1)
  "mod" -> Mod (readRegister $ args !! 0)
               (readValue $ args !! 1)
  "rcv" -> Receive (readRegister $ args !! 0)
  "jgz" -> Jump (readValue $ args !! 0)
                (readValue $ args !! 1)
  where i = take 3 s
        args = splitOn " " $ drop 4 s

type Machine = M.Map Reg Int
type Pc = Int

type Instructions = V.Vector Instruction
step :: Instructions -> (Machine, Pc) -> [Int]
                     -> (Machine, Pc, [Int], Maybe Int)
step ins (m, pc) c = 
  case i of
      Send v -> (m, pc + 1, c, Just $ getValue v)
      Set a b -> (setValue a b, pc + 1, c, Nothing)
      Add a b -> (setValue a $ Value
                             $ (getValue b) +
                               (getValue $ Register a),
                  pc + 1, c, Nothing)
      Mul a b -> (setValue a $ Value
                             $ (getValue b) *
                               (getValue $ Register a),
                  pc + 1, c, Nothing)
      Mod a b -> (setValue a $ Value
                             $ (getValue $ Register a) `mod`
                               (getValue b),
                  pc + 1, c, Nothing)
      Receive v -> (setValue v $ Value $ head c, pc + 1, tail c, Nothing)
      Jump a b -> if (getValue a > 0) then (m, pc + getValue b, c, Nothing)
                                      else (m, pc + 1, c, Nothing)
  where i = ins V.! pc
        getValue (Value i) = i
        getValue (Register r) = fromMaybe 0 $ M.lookup r m
        setValue r v = M.insert r (getValue v) m

run :: Instructions -> Int -> [Int] -> [Int]
run ins i c = run' ins (M.singleton 'p' i, 0) c

run' ins mpc@(_, pc) ch =
  let (m', pc', ch', o) = step ins mpc ch
  in if isJust o
      then fromJust o:(run' ins (m', pc') ch')
      else (run' ins (m', pc') ch')

mutual :: Instructions -> [Int]
mutual ins = rb
    where ra :: [Int]
          rb :: [Int]
          ra = run ins 0 rb
          rb = run ins 1 ra

main = do
    i <- readFile "input18.txt"
    let ins = V.fromList $ fmap readLine $ lines i
    let result = mutual ins
    putStr $ unlines $ map show $ scanl (+) 0 $ fmap (const 1) result
