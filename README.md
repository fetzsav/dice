# DICECII
BETA

Basically just using grid mapping and rust image library to map blocks to an enum based on pixel data obtained from std::images


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

Buildable and usable out of the box as is. Build... run... you got it. 
## Image to Mapped Dice "ASCII" Art






A CLI app that transforms any image into a unique representation using dice faces as "ASCII" art. The app analyzes the grayscale intensity of each pixel in the image and maps it to a corresponding dice face (1-6). Resulting in a dice-based representation of the image.

Usable in it's current state. Potential to release on Linux repos in the future.

- Core logic is built around square images, although have began to veer towards supporting a more dynamic range of input (and output) images.
- Needs alot of code cleanup and orginization
- Possibility to add to Linux repos later as installable package. Maybe a nifty tool someone would like to use to generate wallpapers or something.


*Copyright Fetzer - copyright@fetz.dev*
