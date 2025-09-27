#!/bin/zsh

function hop_chpwd () {
    ~/projects/hop/target/release/main --add "$(pwd)" >/dev/null &!;
}

function h () {
    cmd=$(~/projects/hop/target/release/main --dir "$@");
    cd "${cmd}";
}

typeset -gaU chpwd_functions 
chpwd_functions+=hop_chpwd



