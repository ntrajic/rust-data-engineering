#!/usr/bin/env bash
/home/codespace/.python/current/bin/python -m venv /home/codespace/venv/
source /home/codespace/venv/bin/activate

# LIBTORCH paths
export LIBTORCH=/workspaces/rust-pytorch-gpu-template/libtorch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:${LD_LIBRARY_PATH}