# Hinafont

A library to convert strings to ASCII arts.

## How to Use

First, include this library as a dependency in your project:

```
[dependencies.hinafont]
version = "0.4.0"
git = "https://github.com/shinkou/hinafont"
```

Then import this module as an external crate and use it:

```
extern crate hinafont;

fn main()
{
	println!("{}", hinafont::conv("test"));
}
```
