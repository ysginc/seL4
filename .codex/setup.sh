#!/usr/bin/env bash
#
# SPDX-License-Identifier: BSD-2-Clause
#
# Bootstrap script for Codex Cloud workspaces.
# Installs host dependencies required to configure and build seL4.

set -euo pipefail

if [[ "${EUID}" -eq 0 ]]; then
    SUDO=""
else
    SUDO="sudo"
fi

export DEBIAN_FRONTEND=noninteractive

${SUDO} apt-get update
${SUDO} apt-get install -y --no-install-recommends \
    bash \
    build-essential \
    ccache \
    clang \
    cmake \
    curl \
    device-tree-compiler \
    git \
    gperf \
    libxml2-utils \
    lld \
    ninja-build \
    pkg-config \
    python3 \
    python3-pip \
    python3-setuptools \
    python3-wheel \
    python3-yaml

# Keep pip install separate so apt-only environments still succeed up to here.
python3 -m pip install --user --upgrade pip
