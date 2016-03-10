// An examle which returns an AST rather than a finished result


#[derive(Debug)]
enum Algebra {
    Sum(Box<Algebra>, Box<Algebra>),
    Product(Box<Algebra>, Box<Algebra>),
    Number(u32)
}

impl Algebra {
    fn calculate(&self) -> u32 {
        match self {
            &Sum(ref a, ref b) => a.calculate() + b.calculate(),
            &Product(ref a, ref b) => a.calculate() * b.calculate(),
            &Number(a) => a,
        }
    }

}

use Algebra::*;

#[macro_use]
extern crate earley;
extern crate env_logger;

use earley::*;

fn main() {
    env_logger::init().unwrap();

    let grammar = Grammar::new(vec![
        earley_production!("Sum" => [NonTerminal("Sum"), Terminal("+"), NonTerminal("Product")]         (a, _, b) -> Algebra; { Sum(Box::new(a.get()), Box::new(b.get())) }),
        earley_production!("Sum" => [NonTerminal("Product")]                                            (a) -> Algebra;       { a.get() }),
        earley_production!("Product" => [NonTerminal("Product"), Terminal("*"), NonTerminal("Factor")]  (a, _, b) -> Algebra; { Product(Box::new(a.get()), Box::new(b.get())) }),
        earley_production!("Product" => [NonTerminal("Factor")]                                         (a) -> Algebra;       { a.get() }),
        earley_production!("Factor" => [Terminal("("), NonTerminal("Sum"), Terminal(")")]               (_, a, _) -> Algebra; { a.get() }),
        earley_production!("Factor" => [NonTerminal("Number")]                                          (a) -> Algebra;       { a.get() }),
        earley_production!("Number" => [Terminal("1")]                                                  (result) -> Algebra;  { Number(1) }),
        earley_production!("Number" => [Terminal("2")]                                                  (result) -> Algebra;  { Number(2) }),
        earley_production!("Number" => [Terminal("3")]                                                  (result) -> Algebra;  { Number(3) }),
    ]);

    let input = "1+(2*3+2)";
    let result = grammar.parse(input).expect("failed to parse");

    println!("{}", input);
    println!("{:?}", result);
    println!("{:?}", result.calculate());
}
