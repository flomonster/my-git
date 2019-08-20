# My Git
[![Actions Status](https://github.com/flomonster/my-git/workflows/Build/badge.svg)](https://github.com/flomonster/my-git/actions)

![My Git Logo](https://github.com/flomonster/my-git/blob/master/logo.png)

The purpose of this project is to learn more about git internals and rust
programming language.

The goal of this project is to imitate as possible git behaviours.

## Installation

```
cargo install --git https://github.com/flomonster/my-git.git
```

## Usage

### Help

The most useful command is `my_git --help` which show you all possible commands.

### Config

Note that you can configure my\_git both locally and globally.

```
$ my_git config --global user.name "John Doe"
$ my_git config --global user.email "john.doe@something.com"
```

### Setup a project repository

```
$ cd my-awesome-project/
$ my_git init # This command generates a .my_git directory
```

### Add files content to the index

```
$ my_git add src/some_file.rs
$ my_git add src/some_directory/
```

### Show the working tree status

```
$ my_git status
Changes to be committed:

	new file:   src/some_directory/.gitkeep
	new file:   src/some_file.rs

Untracked files:
  (use "git add <file>..." to include in what will be comitted)

	.my_gitignore
```

### Create a new commit

```
$ my_git commit -m "A message"
```

### Use branches

```
$ my_git branch feature    # Create a new branch
$ my_git branch            # Show all branches
* master
  feature
$ my_git branch -d feature # Delete a branch
```
