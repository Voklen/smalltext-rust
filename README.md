# smalltext
Convert to ₛₘₐₗₗₜₑₓₜ with a small program that follows the Unix philosophy

## Usage
To use smalltext interactively launch it with -i:
```
> smalltext -i
Enter smalltext type to convert to (subscript, superscript, smallcaps)
subscript
Enter text to be converted (ctrl-c to exit):
voklen
ᵥₒₖₗₑₙ
^C
```
You can use smalltext on files:
```
> echo "hello" > greeting
> smalltext --sub greeting
ₕₑₗₗₒ
```
Text can even be piped in:
```
> ls | smalltext --smallcaps
Cᴀʀɢᴏ.ʟᴏᴄᴋ
Cᴀʀɢᴏ.ᴛᴏᴍʟ
ɢʀᴇᴇᴛɪɴɢ
LICENCE
README
ʀᴜsᴛꜰᴍᴛ.ᴛᴏᴍʟ
sʀᴄ
sᴛᴀᴛs
ᴛᴀʀɢᴇᴛ
```

## Install
```
git clone https://github.com/Voklen/smalltext-rust.git
cd smalltext-rust
cargo install
```

If it does not work you may need to add the Cargo bin directory to your PATH variable with:
```
export PATH=~/.cargo/bin:$PATH
```
