# jim

jim is a configuration profile management tool for neovim

## Usage

###To install:

```
cargo install jim
```

###Basic usage:

First configure the default configuration path for jim:
```
jim init </path/to/default/configuration/file>
```

To add profiles:
```
jim add <profile_name> </path/to/configuration/file>
```

To list all available profiles:
```
jim list
```

To run nvim with a specific configuration file:
```
jim <profile_name>
```

To run nvim with the default configuration file:
```
jim
```
