#!/bin/bash -xe

TARGET_VERSION=$1
TARGET_NAME=$2
RELEASE_NAME=logian-$TARGET_VERSION-$TARGET_NAME

rm -rf $RELEASE_NAME
mkdir $RELEASE_NAME
cp target/$TARGET_NAME/release/logian $RELEASE_NAME/logian
cp -r template $RELEASE_NAME/
cp -r plugin $RELEASE_NAME/

zip -r $RELEASE_NAME.zip $RELEASE_NAME
