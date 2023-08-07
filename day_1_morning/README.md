* println! is for string
* {} gives default output, {:?} gives debug, {:#?} is pretty print formatting

Arrays - [T; N];
* A value of the array type [T; N] holds N (a compile-time constant) elements of the same type T. Note that the length of the array is part of its type, which means that [u8; 3] and [u8; 4] are considered two different types.
* e.g. matrix is [[T; N]; N]
* [[0; 3]; 3] holds 3 elements of the same 0-3 array