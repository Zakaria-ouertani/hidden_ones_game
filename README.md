# Hidden ones game

## Build instructions
```
> git clone https://github.com/Zakaria-ouertani/hidden_ones_game
> cd hidden_ones_game
> cargo build --release
```

## How to play
* You're given a square table where each cell contains 0 or 1.
* Initially they're hidden and you have to guess which cells contain 1.

Example: 
```
                                            ┏━━┳━━┳━━┳━━┓
                                            ┃ 1┃ 2┃ 1┃ 1┃ <━ Each number here shows how many 
                                            ┗━━┻━━┻━━┻━━┛    numbers 1s are in each column.
                                       ┏━━┓ ┏━━┳━━┳━━┳━━┓
                                       ┃ 1┃ ┃..┃..┃..┃..┃ A
                                       ┣━━┫ ┣━━╋━━╋━━╋━━┫
                                       ┃ 1┃ ┃..┃..┃..┃..┃ B
And each number here shows how many ━> ┣━━┫ ┣━━╋━━╋━━╋━━┫
numbers 1s are in each row.            ┃ 1┃ ┃..┃..┃..┃..┃ C
                                       ┣━━┫ ┣━━╋━━╋━━╋━━┫
                                       ┃ 2┃ ┃..┃..┃..┃..┃ D <┓  
                                       ┗━━┛ ┗━━┻━━┻━━┻━━┛    ┣━These indicate the coordinates of 
                                             1  2  3  4 ˂━━━━┛ each cell. (Example: A,1 / C,2)
```

The amount of chances you have is the amount of 1s in the table.
The example above has 5.

After using all your chances the game ends and tells you how many mistakes you made and shows your answer with the correct one next to it.
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
 
 ### Playthrough
 ![Playthrough](g.gif)
