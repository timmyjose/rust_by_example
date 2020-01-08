struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(100));
    generic(SGen(100));
    generic(SGen("Hola, Mundo"));
    generic(SGen(true));
    generic(SGen(1 / 2345));
    generic(SGen(SGen(S(A))));

    generic::<char>(SGen('x'));
    generic(SGen('x'));
}
