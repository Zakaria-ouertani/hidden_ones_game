# Hidden ones game

## Build instructions
```
> git clone https://github.com/Zakaria-ouertani/hidden_ones_game
> cargo build --release
```

## How to play
* You're given a square table where each cell contains 0 or 1.
* Initially they're hidden and you have to guess which cell contain 1.

Example: 
```
                                           ┏━━┳━━┳━━┳━━┓
                                           ┃ 1┃ 2┃ 1┃ 1┃ <━ Each number here show how many 
                                           ┗━━┻━━┻━━┻━━┛    numbers 1s are in each column.
                                      ┏━━┓ ┏━━┳━━┳━━┳━━┓
                                      ┃ 1┃ ┃..┃..┃..┃..┃ A
                                      ┣━━┫ ┣━━╋━━╋━━╋━━┫
                                      ┃ 1┃ ┃..┃..┃..┃..┃ B
And each number here show how many ━> ┣━━┫ ┣━━╋━━╋━━╋━━┫
numbers 1s are in each row.           ┃ 1┃ ┃..┃..┃..┃..┃ C
                                      ┣━━┫ ┣━━╋━━╋━━╋━━┫
                                      ┃ 2┃ ┃..┃..┃..┃..┃ D <┓  
                                      ┗━━┛ ┗━━┻━━┻━━┻━━┛    ┣━These indicate the coordinates of 
                                            1  2  3  4 ˂━━━━┛ each cell. (Example: A,1 / C,2)
```

The amount of chances you have is the amount of 1s in the table.
The example above has 5.

After using all your chances the game end and tells you how many mistakes you made and gives u your table with alongside the correct one.
Example
```
 ┏━━┳━━┳━━┳━━┓    ┏━┳━┳━┳━┓
 ┃-1┃ 1┃..┃..┃    ┃0┃1┃1┃1┃
 ┣━━╋━━╋━━╋━━┫    ┣━╋━╋━╋━┫
 ┃..┃..┃..┃..┃    ┃0┃1┃0┃1┃
 ┣━━╋━━╋━━╋━━┫    ┣━╋━╋━╋━┫
 ┃ 1┃-1┃..┃..┃    ┃1┃0┃1┃0┃
 ┣━━╋━━╋━━╋━━┫    ┣━╋━╋━╋━┫
 ┃..┃ 1┃-1┃-1┃    ┃0┃1┃0┃0┃
 ┗━━┻━━┻━━┻━━┛    ┗━┻━┻━┻━┛
Vous avez terminez avec 4 faute(s).
 ```
 
