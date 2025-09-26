#!/bin/zsh

function hop_chpwd () {
    ~/lcode/lrust/projects/hopv2/target/release/main --add "$(pwd)" >/dev/null &!;
}

function h () {
    cmd=$(~/lcode/lrust/projects/hopv2/target/release/main --dir "$@");
    cd "${cmd}";
}

typeset -gaU chpwd_functions 
chpwd_functions+=hop_chpwd



