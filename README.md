# Font Rasterizer
Font Rasterizer is a CLI tool made with the purpose of generating font atlas textures.

## Examples & Usage
Here are a few examples of atlases made using font rasterizer:

![LanaPixel-(w20-h13)](https://github.com/user-attachments/assets/93e68045-1b43-4b95-a8dc-7ed77d3138d2)
![PixelCode-Medium-(w14-h16)](https://github.com/user-attachments/assets/7f2ce19f-0b2f-4e59-827c-99e190b083eb)

And here is an example of what you can do with those textures:

![font-orb](https://github.com/user-attachments/assets/02675c08-e06a-4f6e-8f53-e41d3a0a2b95)




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

## Generating font atlas textures

![demo](https://github.com/user-attachments/assets/1fe692c9-1d8f-4e4a-a5b2-e5ab936c0312)

### Generation Steps:
#### <ins>Choose a font to render.</ins>
When the program starts you will be prompted to choose a font file (otf, ttf or ttc) as a file selector dialog window opens.

#### <ins>Choose rendering height of characters (in pixels, can be decimal).</ins>
This value will determine the space allocated to a character "cell" in the texture atlas, it is really important to have this correct for pixel perfect fonts to render correctly.

#### <ins>Choose a rendering layout.</ins>
Whether the atlas is layed out in a squarish shape atlas or as horizontal/vertical atlas. The "Squarish" shape allows for larger renders.
- Squarish
- Horizontal
- Vertical

#### <ins>Choose a property to sort the character rasterizations by.</ins>
As of now the program only exposes 3 character rasterization properties, but you can also choose not to sort:
- None
- Brightness (a sum of all character pixel values)
- Width (of rasterization pixel grid)
- Height (idem)

#### <ins>Choose to remove duplicate character rasterizations by property.</ins>
Once again the program only exposes 3 character rasterization properties, and you can also choose not to remove any duplicate rasterization:
- None
- Brightness (a sum of all character pixel values)
- Width (of rasterization pixel grid)
- Height (idem)

At this point the program tells you how many duplicates exist by property.

#### <ins>Choose the character rendering direction</ins>
This determines if the characters are layed out left-to-right or top-to-bottom:
- Left to Right
- Top to Bottom

#### <ins>Choose the texture atlas target directory</ins>

## Useful resources & Inspiration
The whole reason I was inspired to develop this tool was Acerola's [font art video](https://www.youtube.com/watch?v=gg40RWiaHRY&t=719s), I wanted to recreate the effect shown in his video but could not find any adequate font texture atlas so I decided to make my own.

### Some of the fonts I used:
- [LanaPixel](https://opengameart.org/content/lanapixel-localization-friendly-pixel-font)
- [Unifont](http://czyborra.com/unifont/updates/unifont.ttf)
- [PixelOperator](https://www.dafont.com/pixel-operator.font)
- [PixelCode](https://qwerasd205.github.io/PixelCode/)





