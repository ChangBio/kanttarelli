# LSM model simulation

This folder contains the LSM model simulation implemented in Rust. It was developed with rustc 1.67.0 version. Currently, the program is in visualization mode. It can be used for batch samples generations. An example of batch generation is in the `run` function in `main.rs`. Results are saved as pickle files in order to analyse them further in Python. Jupyer notebooks, which were used in the analysis are in `jupyter notebooks` subfolder.
Demo is also avaliable under [link](https://fingal.github.io/LSM/index.html).

Visualization aspect of the application is based on a fork of three_d libary that is part of the repository.