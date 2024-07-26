### MINI GREP - limited Grep tool functionality, task given by Rust book
---

**Link to the task**
https://doc.rust-lang.org/beta/book/ch12-00-an-io-project.html

**Requirements**
- rustc - 1.79.0
- cargo - 1.79.0

**Building**:
```bash
cargo build
```

**Use cases**
Say, you have large file and you want to search in the file for some exact string. Example file to test is given in the repository. But you can search from any file only file should be encoded UTF-8
```bash
cargo run -- search_query file.txt
```

If you want to search without case sensitivity, you have to set env variable IGNORE_CASE
```bash
IGNORE_CASE=1 cargo run -- search_query file.txt
```

**Example output**
When minigrep finds results with given query, it prints them to std out:
```text
cargo run -- lorem file.txt

03-20 | The point of using Lorem Ipsum is that it has a more-or-less
06-39 | packages and web page editors now use Lorem Ipsum as their default model text, 
07-19 | and a search for 'lorem ipsum' will uncover many web sites still 
11-42 | There are many variations of passages of Lorem Ipsum available, 
14-21 | to use a passage of Lorem Ipsum, you need to be sure there isn't anything embarrassing 
15-39 | hidden in the middle of text. All the Lorem Ipsum generators 
18-80 | Latin words, combined with a handful of model sentence structures, to generate Lorem Ipsum 
19-39 | which looks reasonable. The generated Lorem Ipsum is therefore
```