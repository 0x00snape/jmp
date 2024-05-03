# jmp
JMP is a simple rust tool that makes easy to navigate to a directory in the file system. Use this tool on my personal space if you want, use it...

## Quick-Install
<h4> Getting rust binary </h4>
<code>git clone<br>cd jmp<br>cargo build --release </code>
<h4> Sourcing the rust binary to .bashrc add this function </h4>
<code>
  jmp() {
	  cd "$(~/jmp/target/release/jmp $1)"  
  }
</code>
