# Calculator Language

This is a programming langauage that makes it extremely straightforward to program mathamatical expressions. 

**Functions**
```
lerp(A, B, T) => A + T * (B - A)
display(lerp(0, 50, 100))
```

**Variables**
```
(5 * 60)^3 -> X
99 -> Y
XY -> Z
display(Z)
```

**Constants**
```
3.14159 -> pi
pi * 2 -> tau
```

**Comments**
```
... display alt character function
dac(120) ... '120' is the alt-code for 'x'
... displays 'x' to the console 
```

**If Patterns**
```
... returns X if X > Y, else returns Y
max(X, Y) => X > Y: X; Y

max(-5, 70) -> A
7 * (5 > A: 1; 2) -> B
```

**Markers and `goto()`**
```
$0               ... marker
I + 1 -> I       ... increment I
I < 10: goto(0)  ... loops 10 times
```

**Complex Functions**
```
sin(X) => degtorad(X) - (degtorad(X)^3 / 3!) + (degtorad(X)^5 / 5!) - (degtorad(X)^7 / 7!)
cos(X) => 1 - (sin(X)^2)
tan(X) => sin(X) / cos(X)
ln(X) => 2 * ((X-1) / (X+1) + (1/3) * ((X-1)/(X+1))^3 + (1/5) * ((X-1)/(X+1))^5 + (1/7) * ((X-1)/(X+1))^7 + (1/9) * ((X-1)/(X+1))^9) 
log(X) => ln(X) / ln(10)
```

**[Example Code](https://github.com/JBrosDevelopment/calc_lang/blob/master/src/calculation.txt)**