Generates a grid of random pixel-art icons.

Compile with Rust: `cargo build --release`

Run `pixel_icons_generator --help` for the available command-line arguments.

# Examples

`pixel_icons_generator -x_mirror`

![](examples/-x.png)

`pixel_icons_generator --x-mirror --width 10 --height 100 --rows 1`

![](examples/--x-mirror_--width_10_--height_100_--rows_1.png)

`pixel_icons_generator -c 1 -r 4 -w 100 -g 12 -p 6 -x -y`

![](examples/-c_1_-r_4_-w_100_-g_12_-p_6_-x_-y.png)

`pixel_icons_generator -u -y`

![](examples/-u_-y.png)

`pixel_icons_generator -u -y -n 100 -k 2`

![](examples/-u_-y_-n_100_-k_2.png)