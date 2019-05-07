# SCWALL TODO/NOTES

# TODO
* [x] implement minimal `shcwell`:
  * [x] fork child bash process and pipe to/from it
  * [x] interrupt after 5 seconds and prompt for scwallpass
  * [ ] if user ctl-c's, run `sl`
* [ ] implement reading from `schwellin`
  * [ ] redirect messages from `/dev/schwellin/xx` to bash
  * [ ] interrupt when messages received on `schwellin`
* [ ] implement `schwell` login
  * [ ] on login, prompt for password
  * [ ] use password to generate pk/sk pair, with sk encrypted to KDF(password)
* [ ] implement `scwall` message sending
  * [ ] parse message and user
  * [ ] retrieve user's pk, encrypt message to it
  * [ ] retrieve user's `schwellin` file descriptor, write cyphertext to it
* [ ] implement `shcwell` message decryption
  * [ ] when bytes recieved on `schwellin`, interrupt, prompt for password
  * [ ] use KDF(password) to unclock sk, decrypt message
  * [ ] pipe message to bash

# SENDER SPECS

* **USAGE:** `scwall -m "my message" -u foo`
* **MAIN ROUTINE**
  * parse argv into message and username
  * find username's pubkey, encrypt message to it (if pubkey not found, show sl piped)
  * parse an  iov (pointer and length of char array) from a string (derrived from argv)
  * loop over users (all uses getutxent(), which contains ut_line/device, we will likely inspect `/dev/shcwelllin`)
    * send encrypted message to *each* user's `shcwellin` fild descriptor
    * handle any errors

# RECEIVER SPECS

* **USAGE:** on login, user enters long-running `shcwell` shell process
* **ENVIRONMENT REQUIREMENTS:**
  * run `shcwell` as daemon
  * set default shell for all users to `schwell`
* **LOGIN ROUTINE:**
  * prompt for new password every time user logs in (with confirmation)
  * enforce very absurd requirements on password (must contain at least 2 prime numbers larger than 666)
  * store hashed password in `/dev/scwall/pwd` (so that passwords may not be reused in subsequent sessions)
  * generates pk/sk pair, encrypts sk to KDF(password)
  * generates a custom file descriptor in `/dev/scwall/` (where each `schwell` gets a `/dev/scwall/shcwellin-XX` FD)
* **RUN LOOP:**
  * fork a command running bash
  * redirect stdin from user to bash, redirect sdout from bash to user
  * on interrupt command: stop redirecting stdout, pause to prompt for password, when password provided, resume redirecting stdout to bash
  * interrupt happens when bytes are written to the file descriptor
* **LOGOUT ROUTINE:**
  * remove `shcwellin` file
  * destroy keystore (?)

# DESIGN QUESTIONS

* [ ] Q: how does shcwell start? (ie: how do we enforce all users use our wrapped shell?)
* [x] Q: how to hijack bash login (to prompt for password on login)?
  * A: rust cli executable that forks bash and pipes to/from it
* [x] Q: how shall scwall write to terminal?
  * how does wall write to tty?
  * should we wrap wall in some way? or... should we replace wall altogether? <-- YES
  * A: scwall writes to a custom file descriptor that schwell (shell wrapper listens to, pipes to bash)

* [x] Q: how do we generate keypairs?
  * A: with adapted forest code
* [x] Q: how to unlock private keys?
  * A: from schwell login prompt

# IMPLEMENTATION NOTES

Helpful breadcrumbs

* dockerized test/dev env w/ debian + 3 ssh users (foo/bar/baz)
* setup error handling https://docs.rs/human-panic/1.0.1/human_panic/macro.setup_panic.html
* https://github.com/hashmismatch/terminal_cli.rs
* https://doc.rust-lang.org/std/process/struct.Command.html

# NOTES ON WALL SOURCE CODE

see: https://github.com/karelzak/util-linux/blob/master/term-utils/wall.c

**Q: how does wall write?**
* 1. read message, write to a buffer..
  * c impl uses an iovec to wrap buffer and track pointer and size (we likely don't need this)
* 2. find users and deliver msgs
  * 0. (implicitly: system must have already called setutxent)
  * loop over all logged-in users (by calling getutxent() in a while loop, which returns utmpptr, which is a utmpx)
  * NOTABLY: this gives us the line/device via utmpptr->ut_line
  * skip users w/o username
  * (there is some stuff about checking if they type of utxent is USER_PROCESS that we think we can ignore)
  * call ttymsg
* 3. ttymsg actually sends stuff, by...
  * consuming:
    * *iov pointer /size_t (represent an array of iovec's -- pointer to cur vec and size of array of vecs) <-- how to construct these?
    * *line (string representing device name) <<--- what is that?
    * tmout timeout for write
    * using writev to write a vector of iov (buffers) to a file descriptor (returning num of bytes written)
    * returning erorr
    * close user db w/ endutxent
