/* It prints:
-2.3 + 0i, -2.1 - 5.2i, -2.2 + 5.2i*/
fn main() {
  struct Complex {
      re: f64,
      im: f64,
  }
  impl std::fmt::Display for Complex {
      fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
          write!(
              f,
              "{} {} {}i",
              self.re,
              if self.im >= 0. { '+' } else { '-' },
              self.im.abs()
          )
      }
  }
  let c1 = Complex { re: -2.3, im: 0. };
  let c2 = Complex { re: -2.1, im: -5.2 };
  let c3 = Complex { re: -2.2, im: 5.2 };
  print!("{}, {}, {}", c1, c2, c3);
}

// このコードは、複素数を表現する構造体 Complex とその表示方法を定義する std::fmt::Display トレイトの実装を含んでいます。
// まず、Complex 構造体が定義されています。この構造体には、実部 (re) と虚部 (im) の2つのフィールドがあり、f64 型で表現されています。
// 次に、std::fmt::Display トレイトの実装が行われています。このトレイトは、fmt メソッドを実装することで、その型のインスタンスを文字列として表示する方法を定義できます。この実装では、write! マクロを使って、実部と虚部を組み合わせた文字列を生成しています。虚部の値が正の場合は +、負の場合は - が付加されます。また、abs() メソッドを使って虚部の絶対値を取得しています。
// 最後に、3つの Complex 型のインスタンスが作成され、それぞれの値が print! マクロで出力されています。

// このコードでは、カスタム型 (Complex) に std::fmt::Display トレイトを実装することで、その型のインスタンスをどのように文字列化するかを定義しています。これにより、println! や print! などの標準的な出力関数で、カスタム型のインスタンスを簡単に出力できるようになります。トレイトの実装は、既存の型に新しい振る舞いを付加する強力な機能です