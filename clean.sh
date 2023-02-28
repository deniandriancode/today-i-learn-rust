#!/usr/bin/env bash

#####################################
# clean.sh -- remove target directory
#####################################

find . -iname target -exec rm -rv {} \;
