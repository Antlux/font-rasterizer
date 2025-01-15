# Font Rasterizer
Font Rasterizer is a tool made with the purpose of generating and rendering font atlas textures. Though Font Rasterizer was primarily intended for pixel perfect fonts, there is no reason why it cannot render any other font that comes in the supported formats (ttf, ttc, otf).

## Examples & Usage

<details>
  <summary>Example renders</summary>
  
  #### [Unifont](http://czyborra.com/unifont/updates/unifont.ttf)
  ![unifont-(16w-16h)-(228H-228V)](https://github.com/user-attachments/assets/34fd3007-cefe-4d00-84f0-427498902604)
  
  #### [LanaPixel](https://opengameart.org/content/lanapixel-localization-friendly-pixel-font)
  ![LanaPixel-(20w-13h)-(53H-18V)](https://github.com/user-attachments/assets/2495817d-2ae0-40d1-8c95-22a6d8b9fa24)
  
  #### [PixelOperator](https://www.dafont.com/pixel-operator.font)
  ![PixelOperator8-(11w-8h)-(17H-12V)](https://github.com/user-attachments/assets/d39bff3b-a620-4ecf-a637-c7a6b3fe3d67)

</details>



<details>
  <summary>Some art I made</summary>

  All examples used textures made using Font Rasterizer.
  
  ![transparent_background_scale_up](https://github.com/user-attachments/assets/cc080961-26f3-4dce-a6e2-ef71a2bf20e4)
  
  ![flow-2](https://github.com/user-attachments/assets/5a39f3d8-18fc-4fc2-9d00-874f26128a65)

  Studies made in Godot

  
  ![font-orb](https://github.com/user-attachments/assets/02675c08-e06a-4f6e-8f53-e41d3a0a2b95)
  
  Study made in [Material Maker](https://www.materialmaker.org/)

</details>


## Using the tool & Requirements
To be able to use Font Rasterizer you need to have a [Rust](https://www.rust-lang.org) installed on your machine. 

### Running the project
First you need to clone the repository as there is no release as of yet.
```bash
  git clone https://github.com/Antlux/font-rasterizer.git
```
Then running the project is as simple running "cargo run" in the project folder.
```bash
  cd font-rasterizer
```
```bash
  cargo run
```

## Inspiration
The whole reason I was inspired to develop this tool was Acerola's [font art video](https://www.youtube.com/watch?v=gg40RWiaHRY&t=719s), I wanted to recreate the effect shown in his video but could not find any adequate font texture atlas so I decided to make my own.




