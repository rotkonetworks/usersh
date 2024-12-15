# usersh

Display user shell information from /etc/passwd in a formatted table.

## Install

Linux/macOS AMD64:
```bash
curl -L "https://github.com/rotkonetworks/usersh/releases/latest/download/usersh-$(uname -s | tr '[:upper:]' '[:lower:]')-amd64" -o usersh && chmod +x usersh && sudo mv usersh /usr/local/bin/
