# mc-cpu-compiler
A simple compiler written for my 8-bit super basic minecraft computer.(br/>

```
0000 | ADD -> r(n)     | add -> r(n)
0001 | SUB -> r(n)     | sub -> r(n)
0010 | r(n) -> a0      | r(n) -> a(n)
0011 | r(n) -> a1      | 
0100 | (n) -> clk ? == | if == jump -> (n)
0101 | (n) -> clk ? != | if != jump -> (n)
0110 | (n) -> a0       | (n) -> a(n)
0111 | (n) -> a1       |
1000 | (n) -> ck       | jump -> (n)
1111 | dummy           | pass
```
