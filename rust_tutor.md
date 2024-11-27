### Install Rust On Linux  
- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`  
    - `cargo --version`  
    - `rustc --version`  
    - `rustdoc --version`  
    - `rustup update`  

> Proxy is preferable.  

### Basic Type  
1. Auto-deduce for variable definition is preset.  
2. Using `0x,0o,0b` to denote hexadecimal,octal and binary number. 
3. Using `as` to achieve type conversion.  
4. You can add a comma on the tail of function parameters, array, enum, etc.


|Type|Explanation|Val|  
|:--:|:--:|:--:|  
|i8, i16, i32, i64, u8, u16, u32, u64|Integer|42, -5i8, 20_922_789_888u64, b'*'|  
|isize, usize|Size the same the machine word||
|f32, f64|Float number|1.8, 3.14f32|  
|bool|Boolean|true, false|  
|char|Unicode char with 32bit width|'\n', '*'|  
|(char, u8, i32)|Tuple|('%', 0x7f, -1)|  
|()|Empty tuple|()|  
|struct S{field:type}|Named struct|S{field:89}|
|struct T(i32, char);|Tuple-like struct|T{120, 'x'}|  
|struct E;|No field|E|  
|enum Attend{OnTime, Late(u32)}|Enumeration|Attend::Late(5), Attend::OnTime|  
|Box<Attend>|Pointer|Box::new(Late(15))|  
|&i32, &mut i32|Reference|&s.field, &mut v|  
|String|UTF-8 string|"roman".to_string()|  
|&str|Reference to String|"roman", &s[0..12]|  
|[f64;4]|Array with fixed length|[b' ';256]|  
|Vec<f64>|Vector with desirable length|vec![1, 2, 3]|  
|&[u8], &mut [u8]|Slice|&v[1..12]|
|&Any, &mut Read|Trait type|&mut file as &mut Read|  
|fn(&str, usize)->isize|Function pointer|String::from|  
|\|para\|{body}|Closure|\|a, b\| a\*a+b\*b|

### String and char  
1. Every char in string occupies a bit if ASCII otherwise several bits(Unicode).  
2. You can view String as Vec<u8> and &str as &[u8]. But only u8 can be converted to char.  



### Ownship  
1. Variable owns its value, being destoryed when out of its scope.  
2. Moving the ownship leads the original variable uninitialized.  
    - For variable in struct or vector, the rules also apply, leading the field or some datas of specific index uninitialized.  
3. Moving happens when passing parameters in functions, constructing tuples and etc,except the datas that implement the copy trait.    

> Copy trait including integer, float, char, bool, and tuple and array in which the data implementes the copy trait.  

#### Reference  
> Multiple readers and single writer.  

1. Deep Reference is allowed in Rust.  

### Life Span  
> Notation for reference  
```rust

```

### Expression and Statement  
> Comma is meaningful  
```rust 
// An expression  
5 * (fahr-32) / 9  
// This is Statement

//if match etc can produce value  
let status = 
    if cpu.temperature <= MAX_TEMP {
        HttpStatus::Ok
    }else{
        HttpStatus::ServerError
    }
```

#### Call a method or function for Generic Type
```rust
//error, < is an operator
Vec<i32>::with_capacity(100);
let ramp = (0..h).collect<Vec<i32>>();

//Using ::<T>, then ok
Vec::<i32>::with_capacity(100);
let ramp = (0..h).collect::<Vec<i32>>();

//Otherwise let rust auto deduce if it works.
return Vec::with_capacity(100); 
```

### Built-in Type Implicit Conversion  
1. `&String` type is auto converted to `&str`  
2. `&Vec<Type>` is auto converted to `&[Type]`  
3. `&Box<Type>` is auto converted to `&Type`  

### Error Handle  
> Result<T,E>, an enumeration  
```rust
//return bool value
result.is_ok();
result.is_err();

//return the corresponding part if exists, otherwise None    
//result is depleted  
result.ok();
result.err();
result.unwrap_or(fallback) //return Success if exists. Otherwise return fallback and discard the errors
result.unwrap_or_else(fallback_fn)

result.unwarp();
result.expect(msg);

//without consuming the result  
result.as_ref().ok();
result.as_mut();
```

#### Propagate errors  
```rust
use std::fs;
use std::io;
use std::path::Path;
pub type Result<T> = result::Result<T,Error>;

fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(),dst_file)?;
    }
    Ok(())
}
```

#### Define Error Type  
```rust
#[derive(Debug,Clone)]
pub struct MyError {
    pub msg: String,
    pub line: usize,
    pub column: usize,
}

use std::fmt;
impl fmt::Display for MyError {
    fn fmt(&self,f :&mut fmt::Formatter) -> Result<(),fmt::Error> {
        write!(f,"{} ({}:{})",self.msg,self.line,self.column)
    }
}
impl std::error::Error for MyErorr {
    fn description(&self) -> &str {
        &self.msg
    }
}
```

### Package and Module

#### cargo  
```shell
cargo clean //delete the target compiled  
cargo test
cargo run
cargo build --release  //debug_assert!() will be ignored
cargo tree
```

#### module components  
- functions 
- type(struct,enum,trait)
- type alias
    - `type Table = HashMap<String,Vec<String>>;`
- impl block
- const variable
    - `pub const ROOM_TEMPERATURE: f64 = 20.0`  
    - `pub static ROOM_TEMPERATURE: f64= 90.0`  
- sub-module
- import statement
- extern block.
```rust
mod{
    components;
}

//this mod is store in a single file with the same file name.
//When encoutering this, Rust will search the file or directory. If both exist or not exist, error will be raised.
pub mod mod_name;

//for a directory, you should create a file mod.rs and add the mod name in this directory to it.  Then others can use it.
//mod.rs in a subdirectory
pub mod mod1;
pub mod mod2;

//Import mod  
use std::collections::{HashMap,HashSet};
use super::*;  //only import the items in parent scope marked with pub, for importing the private items, specifying exactly is needed.
use self::sub_mod::*;

use self::enum_type::*;
```

#### component attribute 
```rust
// This module will be compiled only for android
#[cfg(target_os = "android")]
mod mobile;
```

#### test  
> Normal functions marked with `#[test]`, run `cargo test`  
```rust
//This mod is included only when testing.  
#[cfg(test)]
mod tests{
    fn roughly_equal(a:f64,b:64) -> bool {
        (a - b).abs() < 1e-6
    }
    #[test]
    fn trig_work(){
        use std::f64::PI;
        assert!(roughly_equal(PI.sin(),0.0));
    }
}
```

#### workspace  
```rust
cargo new project
vim cargo.toml
//delete all content and add below
[workspace]
members=["p1","p2"]

//cargo run -p project --bin file_name
```


### Struct  
1. Struct is CamelCase named, and its fields are snake_case named.  
2. Pub struct still has private field.  
```rust
struct GrayscaleMap {
    pixels: vec![0;width*height],
    size: i32,
}
// When creating a strut in functions, we can simply use the varible with the same name to create quickly.
//Partly using key:value is also ok.
fn new_map(size:i32,pixels:Vec<u8>)->GrayscaleMap {
    GrascaleMap{pixels,size}
}
```
3. Using `.. Expr` to copy the same value from another struct(the rest field should possess the copy trait, otherwise ownship exchanging)  
```rust
struct Broom {
    name: String,
    height: u32,
    position: (f32,f32,f32),
    intent: BroomIntent,
}
#[derive(Copy,Clone)]
enum BroomIntent { FetchWater, DumpWater}

fn chop(b:Broom) -> (Broom,Broom) {
    //broom1 get the ownship of the name field of b
    //using .. struct to gain the rest of value when assign
    let mut broom1 = Broom{height: b.height/2, ..b};
    let mut broom2 = Broom{name:broom1.name.clone(),..broom1}
    (broom1,broom2)
}

```

### Enumeration  
> Must use pattern match to access the data.  
```rust
enum Ordering{
    Less,
    Equal,
    Greater,
}
use std::cmp::Ordering::*;
impl someTrait for Ordering{
    ...
}
//define a enum possessing data type  
//default public, a bit different from struct
enum RoughTime{
    InThePast(f64,u32),            //tuple variant
    JustNow,                            //basic unit
    InTheFuture(f64,u32),
    Sphere {center: Point3d, radius: f32}, //struct variant
}

//powerful enumeration
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String,Json>>),
}
```

### Pattern  

#### Pattern match  
```rust
match value {
    pattern => expr,
    ...
    _ => expr,
}

if let pattern = value { ... }else{ ... }
```

|Type|Example|Explanation|  
|:--:|:--:|:--:|  
|Literal|100, "name"|Match exactly|  
|Range|0...1000, 'a'...'k'|Inclusive Range|  
|Wild Card|_|Match anything| 
|Varaible|name|Get the ownship|  
|ref Variable|ref field|Not get the ownship|  
|Sub pattern bind|val @ 0...99|Move the matched to val|  
|Enum|Some(val), None||
|Tuple|(key, value)||
|Struct|Color(r, g, b), Point{x, y}||  
|Reference|&value||  
|Multi-pattern|'a'\|'A'|Only in match|  
|Condition-pattern|x if x\*x <= r2|Only in match|  

```rust
//use other as default match  
let calendar = match settings.get_string("calendar") {
    "chinese" => Calendar::Chinese,
    other => return parse_error("calendar",other),
}
//only care the specific field
match account {
    Account {ref name, ref language, .. } => {
        ui.greet(name,language);
        ui.show_settings(&account);
    }
}

//specify the ref, not ownship happens.
match line_result {
    Err(ref err) => log_err(err),
    Ok(ref mut line) => {
        trim_commnets(line);
        handle(line);
    }
}
//@ mode
rect @ Shape::Rect(..) => optimized_paint(&rect)
```

#### Multiply/if/@ match  
```rust
let at_end = match chars.peek() {
    Some(&'\r') | Some(&'\n') | None => true,
    _ => false,
}

match robot.last_location() {
    Some(point) if self.distance_to(point) < 10 => {....},  //only if expression is true, match successfully  
    None => {...},
}

match chars.next() {
    Some(digit @ '0' ... '9') => read_number(digit,chars),
    _ => (),
}
```

#### Where to use Pattern match  
```rust
//destruct a structure
let Track {album,track_number,title,..} = song;
//destruct a tuple
fn distance_to((x,y):(f64,f64)) -> f64 {...}
//destruct a key & val for hashmap  
for (id,document) in &cache_map {
    println!("Document #{}:{}",id, document.title);
}
```

### Trait and Generic type  
1. Trait is a set of functions, call it by the type implemented the trait or the trait itself.  

```rust
//trait bound, if impl Creature, Visiable is alse needed
trait Creature: Visiable {
    ...
}
//call by trait object
"hello".to_string();
//call by trait name
ToString::to_string("hello");
<str as ToString>::to_string("hello");
```

### Iterator  
```rust
//associated type
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

### Operator Overload  
```rust
pub trait Mul<RHS=Self>{
    type Output;
    fn mul(self, rhs:RHS) -> Self::Output;
}
```


### Useful trait  
1. Def and DerefMut  
2. Default  
    - use `#[derive(Default)]` for struct
3. AsRef and AsMut  
4. From and Into  


### Closure  
1. Fn default ref  
2. FnMut default ref\_mut  
3. Keyword `move` to gain the ownship  


#### Iterator adapter  
> Gain the ownship of iterator.

> Not consuming a iterator, only return a new iterator: What should be done when calling `next()`. 
1. `map` and `filter`, and `filter_map` & `flat_map`     
```rust
let text = "1\nfrond .25 289\n3.14 estuary";
//return Option<_> Some to be kept and None to be discarded
for i in text.split_whitespace().filter_map(|w| f64::from_str(w).ok()) {
    println!("{}",i.sqrt());
}
```
            
2. `take` and `take_while` 
```rust
//fn take(self, n:usize) -> some Iterator<Item=Self::Item>;

//return None when encoutering false predicting, latter returning None as well.  
for header in message.lines().take_while(|l| !l.is_empty()) {
    println!("{}",header);
}
```
3. `skip` and `ship_while`, as a complement to `take`  
```rust
for arg in std::env::args().skip(1) {
    ...
}
```
4. `std::iter::DoubleEndIterator`, `rev`  
5. `fuse` to make sure always return None when encoutering the first None.  
6. `chain`  
```rust
let v:Vec<i32> = (1..4).chain(vec![2,3,4]).collect();
assert_eq!(v,[1,2,3,2,3,4]);
```
7. enumerate  
```rust
//Add an index to the iterator
for (index,band) in bands.into_iter().enumerate() {
    let top =  band_rows * index;
}
```
8. zip  
>  A more generic enumerate adaptor 
```rust
//work the same as enumerate adapter
let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
assert_eq!(v.vec![(0,'A'),(1,'B'),(2,'C'),(3,'D')];
//iteratable obj is ok as well
zip(T:Iterable)
```
9. `by_ref` 
```rust
let message = "To: jumb\r\n
               From: id\r\n
               \r\n
               Ooooh, donuts!!\r\n";
let mut lines = message.lines();
//without take the ownship of lines using by_ref
lines.by_ref().take_while(|l| !l.is_empty()).last();
for body in lines {
    println!("{}",body);
}
```
10. 'cycle`  



#### Other Iterator providers  

|Type or Trait|Exp|Explain|  
|:--|:--|:--|
|std::ops::Range|1..10|[start, end)|  
|std::ops::RangeFrom|1..|Infinity iterator|  
|Option<T>|Some(10).iter()||
|Vec<T>, &[T]|v.windows(16)|A successive slice from left hands, with overlaping|  
||v.chunks(16)|Without overlaping|  
||v.chunks\_mut(16)|Can be modified|  
||v.split(\|byte\| byte & 1 !=0)|Generate the slice matched|  
||v.split\_mut(...)||  
||v.rsplit(...)||  
||v.splitn(n,...)|With the max size limitation|  
|String, &str|s.bytes()|Bytes coded with UTF-8|  
||s.chars()|Chars coded with UTF-8|  
||s.split\_whitespace()|Generate silces splited by whitespace|  
||s.split('/')|Can be splited by char, string or closure|  
||s.matcher(char::is\_numeric)|Generate the slice matched|  

#### Consumer iterator
1. `for` will apply `IntoIterator::into_iter` and call `next`, the most used generic iter func.  
```rust
//like iter
//for a readonly obj, into_iter return a readonly iterator
for ele in &collection {...}

//like iter_mut
//for a single writing obj, into_iter return a writer iterator
for ele in &mut collection {...}

//for a obj without copy trait, into_iter gain the ownships and return iterator
for ele in collection {...}
```
2. `drain` gains the ownship and clears them in the original collection.
```rust
use std::iter::FromIterator;
let mut outer = "Earth".to_string();
let inner = String::from_iter(outer.drain(1..4));

//the elements lose their ownships are deleted
assert_eq!(outer,"Eh");
assert_eq!(inner,"art");
```
3. `count`, `sum` and `product`  
4. `max` and `min`, std::cmp::Ord trait is needed  
5. `max_by(predict)` and `min_by`  
6. `any` and `all`
7. `position` and `rposition` to index   
8. `last`, consuming to the last element.  
9. `find`, find the first item satisfy the closure  
10. `collect` and `from_iter` in trait `FromIterator`  
11. `extend` which implement IntoIterator  

```rust
let v: Vec<i32> = (0..5).map(|i| 1<<i).collect();
v.extend(&[1,2,3,4]);
```
12. `partition`  
```rust
//std::default::Default is needed for the target type
let (living,nonliving):(Vec<&str>,Vec<&str>)
    = things.iter().partition(|name| name.as_bytes()[0] & 1 !=0);
```

### Collection  
1. `Vec`  
```rust
//new a vec
let mut numbers = vec![2];
let mut buffer = vec![0u8;1024];

//clone is needed
let my_copy = buffer[4..12].to_vec();

//all slice method can be used on vec or array  
//access
slice.first() //the reference to the first element  
slice.last()
slice.get(index) //return Some(val) reference, otherwise None  
slice.first_mut()
slice.last_mut()
slice.get_mut(index)

slice.to_vec() //clone the slice 
slice.len()
slice.is_empty()
Vec::with_capacity(n) //create a new vector with capacity n
vec.capacity()
vec.reserve(n) //make sure capacity >= n + vec.len()

//modify
vec.push(value)
vec.pop() //return Option<T>
vec.insert(index,value) //panic when index > vec.len()
vec.remove(index)       //panic when index > vec.len()
vec.trancate(new_len)   //clear the elements in range [new_len..]
vec.clear()
vec.extend(iterable)
vec.split_off(index)    //split it and return the second part
vec.append(&mut vec2)   //vec2 will be empty
vec.drain(range)        //return a iterator which has been removed
vec.retain(test)        //FnMut(&T) -> bool ,if false then remove it
vec.dedup()             //remove the successive duplicated elements

//for vec with vec element
slices.concat() //return a new vec
slices.join(&separator)

//iterate  
slice.iter()
slice.iter_mut()
slice.split_at(index)
slice.split_at_mut(index)
slice.split(predicate)
slice.split_mut(predicate)
slice.splitn(n,predicate)
slice.rsplitn(n,predicate)
slice.chunks(n) //return a iterator with specified chunk length
slice.chunks_mut(n)
slice.windows(n)

let changes = daily_high_temperature
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<_>>();

//swap
slice.swap(i,j) //exchange the slice[i] and slice[j]
vec.swap_remove(i) //remove slice[i] and return it, then move the last to slice[i]

//sort and search
slice.sort() //need Ord trait
slice.sort_by(cmp) //need std::cmp::Ordering
slice.reverse()
slice.contains(&value)

```

### String And Text  
1. char  
```rust
ch.is_numeric()
ch.is_alphabetic()
ch.is_alphanumeric()
ch.is_whitespace()
ch.is_control()

ch.is_lowercase() 
ch.is_uppercase()

//return a iterator
ch.to_lowercase()
ch.to_uppercase()
```
2. String
```rust
//create a String  
String::new()
String::with_capacity(n)
slice.to_string()
iter.collect()

slice.len() //return the length of byte  
slice.is_empty()
slice[range]
slice.split_at(index) //return (slice[..i],slice[i..])
slice.repeat(usize)

//For String
s.push(ch)
s.push_str(slice)
s.extend(iter)
s.insert(i,ch)
s.insert_str(i,slice)

//std::fmt::Write
use std::fmt::Write;
let mut letter = String::new();
writeln!(letter,"Whose {} these are I think I know", "rutabagas")?;
writeln!(letter,"Java is good")?;

//Add<&str> and AddAssign<&str>, +=

//del
s.clear()
s.truncate(n)
s.pop()
s.remove(i) //return the char deleted
s.drain(range) //return the iterator and deleted it ele in the range when iterator is droped
```

#### String Search  
```rust
s.find(ch)
s.find(str)
s.find(predicate)

slice.find(pattern)
slice.rfind(pattern)
slice.contains(pattern)
slice.starts_with(pattern)
slice.ends_with(pattern)
slice.replace(pattern,replacement) //replace all 
slice.replacen(pattern,replacement,n) //replace for n times

//iterate
slice.bytes()
slice.chars()
slice.char_indices()
slice.lines()
slice.split(pattern)
slice.rsplit(pattern)
slice.splitn(n,pattern)
slice.rsplitn(n.pattern)
slice.splite_whitespace()
slice.matches(pattern)
slice.rmatches(pattern)
slice.match_indices(pattern)

//trim
slice.trim()
slice.trim_left()
slice.trim_right()

slice.trim_matches(pattern)
slice.trim_left_matches()
slice.trim_right_matches()

assert_eq!("001990".trim_left_matches('0'),"1990");

//case
slice.to_uppercase();
slice_to_lowercase();

//std::str::FromStr
assert_eq!(usize::from_str("362880"),Ok(362880))
assert_eq!(bool::from_str("true"),Ok(true))

//parse
let address = "f:::::f".parse::<IpAddr>()?;

//access UTF-8
slice.as_bytes() //return &[u8]
slice.into_bytes() //gain the ownship, and return Vec[u8]
```

#### Format String  
1. `format!`  
2. `println!`  
3. `writeln!`  




### IO  
1. Reader(std::io::Read)  
    1. File opened by `std::fs::File::open(filename)`
    2. Stream of `std::net::TcpStream`  
    3. stdin in `std::io::stdin()`  
    4. std::io::Cursor<&[u8]>  
```rust
//buffer is &mut [u8] and this method will read buffer.len() bytes
//This method is low level
reader.read(&mut buffer)

reader.read_to_end(&mut byte_vec) //return io::Result<(usize)>
reader.read_to_string(&mut string)

//for bufreader  
reader.readline(&mut line) //return io::Result<usize>, reach bottom Ok(0) returned
reader.lines()  //return iterator with io::Result<String>

//new BufReader
BufReader::new(reader)
BufReader::with_capacity(size,reader)
```
2. Writer(std::io::Write)  
    1. File opened by `std::fs::File::create(filename)`  
    2. `std::net::TcpStream`  
    3. `std::io::stdout()` and `std::io::stderr()`  
    4. Vec\<u8\>
```rust
write!(std::io::stderr(),"err: for something")?;
writer.write(&buf)
writer.write_all(&buf) //return Result<()>

//call it manually
writer.flush() //wirte all data in buffer to file, return Result<()>
//new BufWriter
let file = File::create("temp.txt")?;
let writer = BufWriter::new(file);
//or BufWriter::with_capacity(size,writer)
```

### Spawn  

### Macro  
The type supported by `macro_rules!`  
|Type|Example|  
|:--|:-|
|expr|2+2,"ubdo",x.len()|  
|stmt|expression|  
|ty|String, Vec<u8>|  
|path|::std::sync::mpsc|  
|pat|pattern|  
|item| 特性项|  
|block| code block|  
|meta| Attribute body|  
|ident|std,Json|  
|tt|token tree: ; >= {},[]|  


### Unsafe Code  



