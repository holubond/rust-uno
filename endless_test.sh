#!/bin/bash

while :
do
	if ! cargo test ; then
	  break
	fi
done