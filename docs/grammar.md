# Grammar for arithmetic expressions

```
  expression ::= term (('+' | '-') term)*
  term ::= factor (('*' | '/' | '%') factor)*
  factor ::= ('+' | '-') factor | '(' expression ')'

```