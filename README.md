# jmp
JMP is a simple rust tool that makes easy to navigate to a directory in the file system. Use this tool on my personal space if you want, use it...

## Quick-Install
<b> Getting rust binary </b>
```bash
git clone https://github.com/0x00snape/jmp.git
cd jmp
cargo build --release 
```

<b> Sourcing the rust binary to .bashrc "add this function" </b>
```bash
  jmp() {
	  cd "$(~/jmp/target/release/jmp $1)"  
  }
```
