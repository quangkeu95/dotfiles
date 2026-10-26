
# The next line updates PATH for the Google Cloud SDK.
if [ -f '/Users/quangkeu95/google-cloud-sdk/path.zsh.inc' ]; then . '/Users/quangkeu95/google-cloud-sdk/path.zsh.inc'; fi

# The next line enables shell command completion for gcloud.
if [ -f '/Users/quangkeu95/google-cloud-sdk/completion.zsh.inc' ]; then . '/Users/quangkeu95/google-cloud-sdk/completion.zsh.inc'; fi

# bun completions
[ -s "/Users/quangkeu95/.bun/_bun" ] && source "/Users/quangkeu95/.bun/_bun"

# bun
export BUN_INSTALL="$HOME/.bun"
export PATH="$BUN_INSTALL/bin:$PATH"

eval "$(starship init zsh)"
source <(fzf --zsh)

alias kctx="kubectl ctx"
alias kns="kubectl ns"
alias k=kubectl
alias vim=nvim
alias nv=nvim
# Git aliases
# Status & Information
alias current_branch="git rev-parse --abbrev-ref HEAD"
alias gst="git status"
alias gss="git status -s"
alias gd="git diff"
alias gdc="git diff --cached"
alias gds="git diff --staged"

# Add & Commit
alias ga="git add"
alias gcm="git commit -m"
alias gca="git commit --amend"
alias gcam="git commit -a -m"
alias gcan="git commit --amend --no-edit"

# Branch Management
alias gb="git branch"
alias gba="git branch -a"
alias gbd="git branch -d"
alias gbD="git branch -D"
alias gco="git checkout"

# Push & Pull
alias gl="git pull"
alias gp="git push"
alias ggp="git push origin HEAD"
alias ggl='git pull origin $(current_branch)'
alias gsc='git sync'
alias ggpf="git push origin HEAD --force-with-lease"

# Log & History
alias glo="git log --oneline"
alias glg="git log --graph --oneline --decorate"
alias glga="git log --graph --oneline --decorate --all"
alias glog="git log --oneline --decorate --graph"

# Stash
alias gsta="git stash"
alias gstp="git stash pop"
alias gstl="git stash list"

# Rebase & Reset
alias grb="git rebase"
alias grbi="git rebase -i"
alias grh="git reset HEAD"
alias grhh="git reset HEAD --hard"
alias gf="git fetch"
alias gfp="git fetch -p"
# Show recent develop branches
alias gbr="git for-each-ref --sort=-committerdate refs/heads/ --format='%(committerdate:relative)%09%(refname:short)'"

# Cargo
alias cb="cargo b"
alias cf="cargo fmt"
alias cc="cargo clean"
alias cr="cargo run"

alias ag="argocd"

alias te="terragrunt"

export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion

# LLVM Configuration
export LIBCLANG_PATH="/opt/homebrew/opt/llvm/lib"
export DYLD_LIBRARY_PATH="/opt/homebrew/opt/llvm/lib:$DYLD_LIBRARY_PATH"

# zsh-autosuggestions
source $(brew --prefix)/share/zsh-autosuggestions/zsh-autosuggestions.zsh
export PATH="$HOME/.local/bin:$PATH"

# Ctrl+G: open/attach tmux session "main"
bindkey -s "^G" "tmux new -A -s main\n"

export NVM_DIR="$HOME/.nvm"
  [ -s "/opt/homebrew/opt/nvm/nvm.sh" ] && \. "/opt/homebrew/opt/nvm/nvm.sh"  # This loads nvm
  [ -s "/opt/homebrew/opt/nvm/etc/bash_completion.d/nvm" ] && \. "/opt/homebrew/opt/nvm/etc/bash_completion.d/nvm"  # This loads nvm bash_completion
export PATH="/Users/quangkeu95/.terragrunt/bin:$PATH"
export PATH="/Users/quangkeu95/.local/share/solana/install/active_release/bin:$PATH"
