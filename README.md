# addpath

addpath — adds a list of directories to `$PATH`, checks for duplicates and
removes empty paths.

## Usage

1. Clone and build 
1. Add the following line to your `~/.bash_profile`:
	
	```bash
	export PATH="`addpath`"
	```
	
	You can either provide a full path to binary in `.bash_profile` or
	create a symlink from a location already available is `$PATH`
1. Add entries in `~/.addpath`:
	
	```bash
	cat << EOF > ~/.addpath
	/opt/some/software/bin
	/home/user/.cargo/bin
	EOF
	```
1. You may need to log out of your session and log in again to see 
	changes.
