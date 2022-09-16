This is just a guide for the compiler making process.

# Compiler Spec

- [Compiler Spec](#compiler-spec)
	- [Definitions](#definitions)
		- [Ref](#ref)
		- [Val](#val)
		- [Loops](#loops)
			- [Starting loops](#starting-loops)
		- [Ending loops](#ending-loops)
		- [Scopes](#scopes)
			- [Starting scopes](#starting-scopes)
			- [Ending scopes](#ending-scopes)
		- [*VR](#vr)
		- [IncVR](#incvr)
		- [DecVR](#decvr)
		- [ResVR](#resvr)
		- [OppVR](#oppvr)
		- [Deref](#deref)
		- [CharFn](#charfn)
		- [StrFn](#strfn)
		- [ShiftR](#shiftr)
		- [ShiftL](#shiftl)
		- [PrintVRChar](#printvrchar)
		- [AskInt](#askint)
		- [ThrowCode](#throwcode)
		- [PrintVRInt](#printvrint)
		- [PrintNStr](#printnstr)
		- [PrintLastInpOrAsk](#printlastinporask)
		- [AskStr](#askstr)
		- [Comment](#comment)
	- [Transcriptions](#transcriptions)
					- [Bottom notes:](#bottom-notes)

## Definitions

### Ref

The reference (pointer) is the index inside the current scope. It can be operated on like a pointer. For example, one could increase the pointer by one using the following operation: `&+`. To change the value of a value next to the current cell one could use: `&+$+`.

The reference starts at 0 in the global scope, when it enters a custom scope, it will not necessarily start at zero.

### Val

The value is the current value that the pointer points to, one can operate on the value of the current cell by using: `$`, one could operate on the next cell by using: `&+$`

The value is a number between 0 and 255 which can be displayed as an ASCII character (`.`) or a number (`#`).

### Loops

#### Starting loops

The number of times a loop is repeated is determined by the entering cell. the simplest loop one could do (that works.) is: `$+[$-]` which enters a one-time loop, which removes the value, to be 0.

A loop isn't considered its own cell (or group of cells), only a mere calculation. So the address before a loop is just the address after the loop minus one.

### Ending loops

After the loop has ended, it just continues on the next cell.

### Scopes
<sup>[1](#bottom-notes)</sup>

#### Starting scopes

A scope is comparable to a loop, but the scope takes its own cell.
A scope is an array of length depending on the cell before, it's joined by the ends, so doing a `>` operation on a scope would be `[1,2,3,4]` to `[2,3,4,1]`.

For scopes **other** than the global scope, we can call them *"private"*

#### Ending scopes

A scope ends when the pointer is out of it, you can re-reference the scope from out of the scope, re-entering the scope while the reference is in it.

### *VR

Any token with a VR at the end of the name means that it's an operation that could be operated on a **V**alue or a **R**eference

### IncVR

Increments the value or reference.

### DecVR

Decrements the value or reference

### ResVR

Resets the value or reference (to 0)

### OppVR

Gets the opposite of the value / reference. The opposite is X - 255, e. g. the opposite of 3 is 252.

### Deref

Dereferences a reference **or** sets value to address modulo 255.

### CharFn

Executes the Char Algorithm<sup>[1](#bottom-notes)</sup>

### StrFn

Stringifies the value or address (changing to the ASCII value.) For example, 3 would be "3", with an ordinal of 51.

### ShiftR

Shifts right, if the operation is in the global scope this will create a zero at the first position, if the operation is in a private scope, all the numbers are shifted to the right while the ends are joined.

### ShiftL

Shifts left, same as [ShiftR](#shiftr) but destroying the first position in the global scope.

### PrintVRChar

Prints the ASCII character corresponding with the current value / address.

### AskInt

Asks for an integer input between 0 and 255.

### ThrowCode

Throws an error, the current value / address would be the code.

### PrintVRInt

Prints the current value / address as an integer.

### PrintNStr

Prints the next `n` cells as a string, with `n` being determined by the current value / address.

### PrintLastInpOrAsk

Prints the last input, if there's not a last input, asks for another one and prints it.

### AskStr

Asks for an ASCII string and stores it, starting at the current one.

### Comment

All characters that aren't valid are considered comments, and should be ignored.

## Transcriptions

Transcripting these into C is more complex, as we have to manipulate the input in some ways, so I'm going to declare some principles.

1. All operations must have a subject.

In the IR we're going to change the code in a way that, for example, `&++` changes to `&+&+`.

2. All scopes must be a single cell.

As every index in the.

###### Bottom notes:

1. I'm calling "scope" to a sliced array scope because of simplicity.

2. The Char Algorith may be found in the original specs.