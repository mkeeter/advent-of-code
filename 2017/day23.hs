import Text.Read
import Data.Maybe
import Data.Char
import Data.List.Split
import qualified Data.Vector as V
import qualified Data.Map as M

type Reg = Char
data Value = Value Int | Register Reg deriving Show
data Instruction = Set Reg Value
                 | Sub Reg Value
                 | Mul Reg Value
                 | Jump Value Value deriving Show
readRegister = head . dropWhile isSpace
readValue s =
  let r = Register $ readRegister s
      v = Value <$> readMaybe s
    in fromMaybe r v
readLine :: String -> Instruction
readLine s = case i of
  "set" -> Set (readRegister $ args !! 0)
               (readValue $ args !! 1)
  "sub" -> Sub (readRegister $ args !! 0)
               (readValue $ args !! 1)
  "mul" -> Mul (readRegister $ args !! 0)
               (readValue $ args !! 1)
  "jnz" -> Jump (readValue $ args !! 0)
                (readValue $ args !! 1)
  where i = take 3 s
        args = splitOn " " $ drop 4 s
type Machine = M.Map Reg Int
type Pc = Int
type Instructions = V.Vector Instruction
step' :: (Machine, Pc, Int) -> Instructions
                           -> (Machine, Pc, Int)
step' (m, pc, rs) ins = case i of
  Set a b -> (setValue a b, pc + 1, rs)
  Sub a b -> (setValue a $ Value
                         $ (getValue $ Register a) - 
                           (getValue b),
              pc + 1, rs)
  Mul a b -> (setValue a $ Value
                         $ (getValue b) *
                           (getValue $ Register a),
              pc + 1, rs + 1)
  Jump a b -> if (getValue a /= 0) then (m, pc + getValue b, rs)
                                   else (m, pc + 1, rs)
  where i = ins V.! pc
        getValue (Value i) = i
        getValue (Register r) = fromMaybe 0 $ M.lookup r m
        setValue r v = M.insert r (getValue v) m

run :: Instructions -> [(Machine, Pc, Int)]
run ins = run' (
    M.singleton 'a' 1,
    0, 0)
  where run' mpc@(_, pc, i) = if (pc < 0 || pc >= V.length ins)
                              then [mpc]
                              else mpc:(run' (step' mpc ins))
main = do
    f <- readFile "input23.txt"
    let d = unlines $ map (\(m,p,i) -> show (p + 1,m))
                    $ take 10000000 $ run $ fmap readLine $ V.fromList $ lines f
    putStr d

-- 3401 is wrong
-- e goes to b  steps to increment, resets to 2
-- d goes to b steps to increment, resets to 2
-- b takes 1000 steps to increment (in steps of 17)
