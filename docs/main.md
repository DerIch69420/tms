# Tmux Script Documentation

## File
Create a file with the .tms extension (optional)

## Content
Add the following shebang
```
#!/usr/bin/env tms
```

## Comments
Start a line with "//" to make it a comment
```
// This is a comment
```

## Bash commands
Start a line with ```bash``` to execute the following content as a bash command
```
bash "mkdir new-dir"
```

## New Session
Add the following line to create a new session named my-session
```
session "new-session"
```

## Attach to session
```
attach
```

## Windows
```
window "code"
window "git"
```

## Execute it
```
chmod +x tmux.tms 
./tmux.tms
```
