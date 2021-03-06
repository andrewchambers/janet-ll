#! /bin/sh

set -eux

version="0.4.1"

if ! test -d ./janet/.git
then
  rm -rf ./janet
  git clone https://github.com/janet-lang/janet
fi
cd ./janet
git checkout master
git pull
git checkout "v${version}"
git clean -fxd
make amalg
mkdir -p ../csrc
cp build/janet.c ../csrc/
cp build/janet.h ../csrc/
cp src/include/janetconf.h ../csrc/
