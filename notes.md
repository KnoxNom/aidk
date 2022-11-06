# Notes

### Error E0277 =P 5:0
```rust
#[derive(Debug)]
```
#### If that is not kept before the struct it will show error E0277.

### Search for a pattern in a file and display the lines that contant it 6:0
```rust
#[derive(Parser)]
```

### The pattern type 8:0
```rust
pattern: String,
```

### The path is a PathBuf 9:0
```rust
path: std::path::PathBuf,
```

### Loading... 16:0
```rust
println!("Loading aidk...hope it fails lol");
```

### Getting the path 24:0
```rust
let path = "test.txt";
```

### Reading the file 25:0
```rust
let content = std::fs::read_to_string(path)
```

### Bloody erros TwT 26:0
```rust
    .map_err(|err| Error(format!("Annnnd the error is `{}`: {} TwT", path, err)))?;
```

### Welcome screen 28:0
```rust
println!("Welcome to hell uvu\n");
```

### Make a variable to store response 30:0
```rust
let mut word = String::new();
```

### Reading input 32:35
```rust
io::stdin()
    .read_line(&mut word)
    .expect("Failed to read line x-x");
```

### Clear the terminal 36:0
```rust
println!("{esc}[2J]{esc}[1;1H", esc = 27 as char);
```

### Show output 38:0
```rust
println!("{}", word);
```

### End screen 40:0
```rust
pirntln!("Enjoy you're stay <3\n");
```

### Print out file content 42:0
```rust
println!("Oh ya, the file contents are {:#?} :^", content);
```
