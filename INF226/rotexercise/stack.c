int main(int argc, char **argv) {
    f();
    return 0;
}

int f() {
    int a = 5;
    // A
    return g(a);
}

int g(int x) {
    int t1 = h();
    // B
    int t2 = m();
}

int h() {
    return 42;
}

int m() {
    int a = 2, b = 3, c = 8;
    // C
    return a+b+c;
}

