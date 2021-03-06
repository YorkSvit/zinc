# Bitwise operators

The `|=`, `^=`, `&=`, `<<=`, `>>=` shortcut operators perform the operation
and assign the result to the first operand. The first operand must be a mutable memory location
like a variable, array element, or structure field.

> For now, bitwise operators are allowed for constants only. Witness data will be covered soon.

#### Bitwise OR

`|` and `|=` are binary operators.

**Accepts**
1. Integer expression (any type except `field`)
2. Expression of the operand 1 type

**Returns** an integer result of the same type.

#### Bitwise XOR

`^` and `^=` are binary operators.

**Accepts**
1. Integer expression (any type except `field`)
2. Expression of the operand 1 type

**Returns** an integer result of the same type.

#### Bitwise AND

`&` and `&=` are binary operators.

**Accepts**
1. Integer expression (any type except `field`)
2. Expression of the operand 1 type

**Returns** an integer result of the same type.

#### Bitwise shift left

`<<` and `<<=` are binary operators.

**Accepts**
1. Integer expression (any type except `field`)
2. Constant integer expression

**Returns** an integer result of the operand 1 type.

#### Bitwise shift right

`>>` and `>>=` are binary operators.

**Accepts**
1. Integer expression (any type except `field`)
2. Constant integer expression

**Returns** an integer result of the operand 1 type.

#### Bitwise NOT

`~` is an unary operator.

**Accepts**
1. Integer expression (any type except `field`)

**Returns** an integer result.
