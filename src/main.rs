/// `n`에서 시작하는 콜라츠 수열의 길이를 결정합니다.
fn collatz_length(mut n: u32) -> (u32, Vec<u32>) {  
  let mut v: Vec<u32> = Vec::new();
  v.push(n);
  let mut cnt: u32 = 1;
  loop {
    if n == 1 {
      break (cnt, v)
    } else if n % 2 == 0 {
      n /= 2;    
      v.push(n);
      cnt += 1;
    } else {
      n = n * 3 + 1;
      v.push(n);
      cnt += 1;
    }
  }
}

fn main() {
  let n = 10;
  let (a, b) = collatz_length(10);
  println!("cnt({n})={} vec = {:?}", a, b);
}