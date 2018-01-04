# rusty\_pixels 

rusty\_pixels is a Rust port of satyarth's python application [pixelsort](https://github.com/satyarth/pixelsort).
Some features are currently missing. This README is updated to reflect that.

### What is Pixel Sorting?

Have a look at [this post](http://satyarth.me/articles/pixel-sorting/) or [/r/pixelsorting](http://www.reddit.com/r/pixelsorting/top/)

### Usage

From the command line (on Linux):

```
git clone https://github.com/monuszko/rusty_pixels.git
cd rusty_pixels 
cargo build --release
./target/release/rusty_pixels %PathToImage% [options]
```

#### Parameters:

Parameter           | Flag  | Description
--------------------|-------|------------
Interval function   | `-i`  | Controls how the intervals used for sorting are defined. See below for more details and examples.
Output file         | `-o`  | Path of output file. At the moment `test.png` is the default.
Randomness          | `-r`  | What percentage of intervals *not* to sort. 0 by default.
Threshold (lower)   | `-t`  | How dark must a pixel be to be considered as a 'border' for sorting? Takes values from 0-1. 0.25 by default. Used in `edges` and `threshold` modes.
Threshold (upper)   | `-u`  | How bright must a pixel be to be considered as a 'border' for sorting? Takes values from 0-1. 0.8 by default. Used in `threshold` mode.
Char. length        | `-c`  | Characteristic length for the random width generator. Used in mode `random`.
Sorting function    | `-s`  | Sorting function to use for sorting the pixels.

#### Interval Functions

Interval function   | Description
--------------------|------------
`random`            | Randomly generate intervals. Distribution of widths is linear by default. Interval widths can be scaled using `clength`.
`threshold`         | Intervals defined by lightness thresholds; only pixels with a lightness between the upper and lower thresholds are sorted.
`waves`             | Intervals are waves of nearly uniform widths. Control width of waves with `clength`.


#### Sorting Functions

Sorting function    | Description
--------------------|------------
`lightness`         | Sort by the lightness of a pixel according to a HSV representation.
`hue`               | Sort by the hue of a pixel according to a HSV representation.
`minimum`           | Sort on the minimum RGB value of a pixel (either the R, G or B).

#### Authors

Marek Onuszko - this Rust port
satyarth - the Python version

#### TODO ####

Threaded execution
