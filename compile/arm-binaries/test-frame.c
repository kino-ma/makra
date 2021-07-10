int f() {
    int x = 10;
    int y = g();
    int z = x + y;
    int a = 1, b = 1, c = 1;
    return z;
}

int g() {
    int y = 20;
    return y;
}
