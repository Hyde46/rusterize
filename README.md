# Rusterize

<p align="center">
      <a href="https://travis-ci.org/Hyde46/rusterize" alt="Travis">
        <img src="https://travis-ci.org/Hyde46/rusterize.svg?branch=master" /></a>
   <a href="https://github.com/Hyde46/rusterize/actions" alt="GithubActions">
        <img src="https://github.com/Hyde46/rusterize/workflows/Rust/badge.svg" /></a>
</p>
Rusterize is a rasterizer written in rust. 

A toy implementation of rasterization algorithm, also offering ray tracing implementation as a point
of comparison. 

_Work in Progress_.

## Prerequisities

Before you begin, ensure you have met the following requirements:
* You have installed 'rust > 1.38.0'
* You have a `Linux/Mac` machine. Windows is not tested.

## How to build

Building the rusterize, it's quiet easy:

Linux and macOS:
```
$ cargo build --release
```

and the executable will be in target/release/rusterize.

## Contributing
To contribute, follow these steps:

1. Fork this repository.
2. Create a branch: `git checkout -b <branch_name>`. 
3. Make your changes and commit them: `git commit -m '<commit_message>'`
4. Push to the original branch: `git push origin rusterize/<location>`
5. Create the pull request.
6. Make sure Travis build is not failing.

Alternatively see the GitHub documentation on [creating a pull request](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request).

## Contact 

If you want to contact me you can reach me at <denis.heid@protonmail.com>.

## License 

This project uses the following license: [MIT](https://github.com/Hyde46/rusterize/blob/master/LICENSE).

