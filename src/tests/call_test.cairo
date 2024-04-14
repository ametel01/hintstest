use hintstest::oracle::{Request, SqrtOracle};

#[test]
fn sqrt_test() {
    let num = 1764;

    let request = Request { n: num };
    let result = SqrtOracle::sqrt(request);

    assert!(result.n * result.n == num);
}
