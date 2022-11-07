# Notes

## [main.rs](src/main.rs)

### [Error E0277 =P 5:0](src/main.rs#L5)
```rust
#[derive(Debug)]
```
#### If that is not kept before the struct it will show error E0277.

### [Search for a pattern in a file and display the lines that contant it 6:0](src/main.rs#6)
```rust
#[derive(Parser)]
```

### [The pattern type 8:0](src/main.rs#8)
```rust
pattern: String,
```

### [The path is a PathBuf 9:0](src/main.rs#L9)
```rust
path: std::path::PathBuf,
```

### [Loading... 16:0](src/main.rs#L16)
```rust
println!("Loading aidk...hope it fails lol");
```

### [Getting the path 24:0](src/main.rs#L24)
```rust
let path = "test.txt";
```

### [Reading the file 25:0](src/main.rs#L25)
```rust
let content = std::fs::read_to_string(path)
```

### [Bloody erros TwT 26:0](src/main.rs#L26)
```rust
    .map_err(|err| Error(format!("Annnnd the error is `{}`: {} TwT", path, err)))?;
```

### [Welcome screen 28:0](src/main.rs#L28)
```rust
println!("Welcome to hell uvu\n");
```

### [Make a variable to store response 30:0](src/main.rs#L30)
```rust
let mut word = String::new();
```

### [Reading input 32:34](src/main.rs#L32#L34)
```rust
io::stdin()
    .read_line(&mut word)
    .expect("Failed to read line x-x");
```

### [Clear the terminal 36:0](src/main.rs#L36)
```rust
println!("{esc}[2J]{esc}[1;1H", esc = 27 as char);
```

### [Show output 38:0](src/main.rs#L38)
```rust
println!("{}", word);
```

### [End screen 40:0](src/main.rs#L40)
```rust
pirntln!("Enjoy you're stay <3\n");
```

### [Print out file content 42:0](src/main.rs#L42)
```rust
println!("Oh ya, the file contents are {:#?} :^", content);
```

### [Make a variable to open the file 46:0](src/main.rs#L46)
```rust
let f = File::open("test.txt")?;
```

### [Make a new variable to read 47:0](src/main.rs#L47)
```rust
let mut reader = BufReader::new(f);
```

### [Make another new variable with the type string 49:0](src/main.rs#L49)
```rust
let mut line = String::new();
```

### [Another one to read the line and output length 50:0](src/main.rs#L50)
```rust
let len = reader.read_line(&mut line)?;
```

### [Finally print out the message and length in bytes 51:0](src/main.rs#L51)
```rust
println!("Damn the first line is {} bytes long XD", len);
```
