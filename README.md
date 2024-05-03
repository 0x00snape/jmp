# jmp
JMP is a simple rust tool that makes easy to navigate to a directory in the file system. Use this tool on my personal space if you want, use it...

## Quick-Install
<h2> Getting rust binary </h2>
<code>
git clone 
cd jmp 
cargo build --release 
</code>
<h2> Sourcing the rust binary to .bashrc add this function </h2>
<code>
  jmp() {
	  cd "$(~/jmp/target/release/jmp $1)"  
  }
</code>
