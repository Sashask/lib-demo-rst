use rand::Rng;
pub fn gen_ran() -> u8 {
  //乱数生成器を作成
  let mut rng = rand::thread_rng();
  //乱数を生成（u8なので0~255)
  let n: u8 = rng.gen();
  //生成した乱数を返す
  n
}
