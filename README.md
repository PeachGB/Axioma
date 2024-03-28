# Teorema
**THIS LANGUAGE IS IN PROCESS! IS NOT JUST UNSTABLE BUT ANYTHING CAN CHANGE AT ANY MOMENT, USE IT AT YOUR OWN RISK**

Teorema is a functional programming language based on diverse mathematical notation and lamda calculus.
The intended final product aims to look like a fusion between Haskell and some [array programming languages like APL](https://en.wikipedia.org/wiki/APL_(programming_language))

## Milestones

- [] Turing Complete
- [] Compiled to native code 
- [] Self-hosted
- [] a Std-lib  

## About The language
Teorema reads the program line by line,
the line statement ends with the new line so you dont have to put semicol or anything like that

for now this are the things you can do with Teorema 

Assign variables(variables are inmutable):
```teorema 
x := 34

h := 4 + x + y

h$    -- you print values with (Value)$
```
Conditional variable assignement
```teorema
h := 5 --h = 5 
7 > h -> x:= 6 -- if 7 > h then x = 6
```
