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
bash echo Hi!
bash "mkdir new-dir"
```

## New Session
Add the following line to create a new session named my-session
```
session my-session
session "new-session"
```
