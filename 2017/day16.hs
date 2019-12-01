import Data.List.Split
import Data.Tuple
import qualified Data.Vector as V
import Data.Maybe

data Move = Spin Int | Exchange Int Int | Partner Char Char deriving Show

readMove :: String -> Move
readMove ('s':i) = Spin $ read i
readMove ('x':s) = let (a:b:[]) = read <$> splitOn "/" s in Exchange a b
readMove ('p':s) = let (a:b:[]) = head <$> splitOn "/" s in Partner a b

applyMove :: V.Vector Char -> Move -> V.Vector Char
applyMove ps (Spin i) = uncurry (V.++) $ swap $ V.splitAt (length ps - i) ps
applyMove ps (Exchange i j) = ps V.// [(i, ps V.! j), (j, ps V.! i)]
applyMove ps (Partner i j) =
  let i' = fromJust $ V.elemIndex i ps
      j' = fromJust $ V.elemIndex j ps
  in ps V.// [(i', ps V.! j'), (j', ps V.! i')]

cmd = V.toList . foldl applyMove (V.fromList ['a'..'p']) . V.fromList . take 1000000000 . cycle . fmap readMove . splitOn "," <$> readFile "input16.txt"
main = do cmd' <- cmd
          putStrLn cmd'
