# mini-grep
A command line gadget by rust

## Examples 🌰
### 1. Normal use
Input:
```bash
cargo run cold rainy.txt
```
Output:
```
There are 2 matching results
The day is cold,and dark,and dreary; 
My life is cold and dark and dreary;
```
### 2. No result
Input:
```bash
cargo run apple rainy.txt
```
Output:
```
There is no matching results
```
### 3. Lack of parameters
Input:
```
cargo run
```
Output:
```
The arguments is not enough
```
### 4. Non-existent file
Input:
```
cargo run test test.txt
```
Output:
```
Application error: No such file or directory (os error 2)
```
