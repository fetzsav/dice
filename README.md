# DICECII
BETA

Basically just using grid mapping and rust image library to map blocks to an enum based on pixel data obtained by image crate.


To do:
- rewrite
- licensing
- rearrange and clean code into library rust crate for potential other use cases (high level... using other images than "dice".
- rewrite
- add more "use-case" functions, rearrange app accordingly.
- fix tons of 💩
- rewrite


<details> 
  <summary>Image Examples (click me)</summary>
<img src='https://github.com/user-attachments/assets/7c8fa96f-48aa-4167-ba1a-de70a5e9294e' alt='original' width="400" height="400">
<img src="https://github.com/user-attachments/assets/cf6634fa-03a6-4dfb-9b82-5f8ef432c639" alt="Dice output 1" width="400" height="400">
<img src="https://github.com/user-attachments/assets/401aa603-b861-43db-9624-ba8cc9c13e7e" alt="Dice output 2" width="400" height="400">
<img src='https://github.com/user-attachments/assets/7f82a1df-bc2f-431b-9082-294b55bc9ace' alt='white dice' width="400" height="400">
</details>

Buildable and usable CLI app out of the box as is. Build... run... you got it. 
Works best with square images... 2048x2048+ ideally. Reccomended 16 or 32 dice. 
Has support for custom output sizes. Fills blank areas with background dice and centers image without distorting.



*Copyright Fetzer - copyright@fetz.dev*
