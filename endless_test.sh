#!/bin/bash

i=0
while :
do
	if ! cargo test ; then
	  break
	fi
	((i++))
	echo $i"# tests run"
done