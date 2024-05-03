# jmp
JMP is a simple rust tool that makes easy to navigate to a directory in the file system. Use this tool on my personal space if you want, use it...

## Quick-Install
<h4> Getting rust binary </h4>
''' bash
git clone
cd jmp
cargo build --release '''
<h4> Sourcing the rust binary to .bashrc add this function </h4>
'''bash
  jmp() {
	  cd "$(~/jmp/target/release/jmp $1)"  
  }
'''
