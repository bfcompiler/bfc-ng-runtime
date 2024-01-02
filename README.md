# Runtime
### Symbols table:
1. \> = 000b
2. < = 001b
3. \+ = 010b
4. \- = 011b
5. . = 100b
6. , = 101b
7. [ = 110b
8. ] = 111b

### Binary format:
A singluar byte lays out either 1 or 2 brainf**k instructions, and is laid out as follows<br/>
"AA XXX YYY"<br/>
Each value of A represents if each or both instructions are used, with the following examples:
1. 00: None used
2. 01: Right-sided instruction used
3. 11: Left-sided and right-sided instruction used
4. 10: Left-sided instruction is used (this should never happen as it goes right to left)

The X values represent the byte operation from the symbols table<br/>
Same with the Y values but for the other operation<br/>
If any zeroes are detected in these identification bits, it will be registered as the last operation in the script <br/>


### Internal Global/Static Variables
1. VAR_BFBM / Brainf\*\*k Binary Memory: How many memory cells to allocate
2. VAR_BFB / Brainf\*\*k Binary Bytes: Pointer to the bytes that the brainf\*\*k interpreter is going to execute
3. VAR_BFBR / Brainf\*\*k Binary Raw Bytes: The raw bytes that the brainf\*\*k interpreter is going to execute 

### Docker commands
In the current directory simply run the following commands with docker installed to build<br/>
```console 
sudo docker build -t bfc-ng-runtime . --progress=plain
```
<br/>

```console
sudo docker run --name bfc-ng-runtime --mount type=bind,source="$(pwd)"/build,target=/mount/ bfc-ng-runtime:latest
```