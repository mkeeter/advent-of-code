def surf(w,h,l):
    a = l*w
    b = w*h
    c = h*l
    return 2*(a + b + c) + min(a, b, c)

print sum([surf(*(map(int, line.split('x')))) for line in
           open('2a.dat').readlines()])
