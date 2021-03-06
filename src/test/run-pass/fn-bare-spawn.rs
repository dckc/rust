// This is what the signature to spawn should look like with bare functions

fn spawn<T: Send>(val: T, f: extern fn(T)) {
    f(move val);
}

fn f(+i: int) {
    assert i == 100;
}

fn main() {
    spawn(100, f);
}
